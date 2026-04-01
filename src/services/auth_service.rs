use crate::{
    dto::auth::{AuthResponse, LoginRequest, RegisterRequest},
    error::{AppError, AppResult},
    models::user::User,
    repository::user_repository,
    state::AppState,
};
use axum::http::HeaderMap;
use bcrypt::{hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const TOKEN_LIFETIME_SECONDS: i64 = 60 * 60 * 24;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct JwtClaims {
    sub: String,
    email: String,
    exp: usize,
    iat: usize,
}

pub fn validate_register_request(payload: &RegisterRequest) -> AppResult<()> {
    if !payload.email.contains('@') {
        return Err(AppError::Validation("email must be valid".to_string()));
    }

    if payload.full_name.trim().len() < 2 {
        return Err(AppError::Validation(
            "full_name must be at least 2 characters".to_string(),
        ));
    }

    if payload.password.len() < 10 {
        return Err(AppError::Validation(
            "password must be at least 10 characters".to_string(),
        ));
    }

    Ok(())
}

pub async fn register_user(state: &AppState, payload: RegisterRequest) -> AppResult<AuthResponse> {
    if user_repository::find_by_email(&state.db, &payload.email)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict("email already exists".to_string()));
    }
    let password_hash = hash(payload.password, state.config.bcrypt_cost)?;
    let api_key = generate_api_key();
    let stored = user_repository::create_user(
        &state.db,
        payload.email.trim(),
        payload.full_name.trim(),
        &password_hash,
        &api_key,
    )
    .await?;

    build_auth_response(state, stored.into())
}

pub async fn login_user(state: &AppState, payload: LoginRequest) -> AppResult<AuthResponse> {
    let stored = user_repository::find_by_email(&state.db, payload.email.trim())
        .await?
        .ok_or(AppError::Unauthorized)?;

    let password_ok = verify(payload.password, &stored.password_hash)?;
    if !password_ok {
        return Err(AppError::Unauthorized);
    }

    build_auth_response(state, stored.into())
}

pub async fn user_from_bearer(state: &AppState, headers: &HeaderMap) -> AppResult<User> {
    let auth_header = headers
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .or_else(|| auth_header.strip_prefix("bearer "))
        .ok_or(AppError::Unauthorized)?;

    let claims = decode_token(token, &state.config.jwt_secret)?;
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized)?;

    let stored = user_repository::find_by_id(&state.db, user_id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    Ok(stored.into())
}

pub async fn user_from_api_key(state: &AppState, headers: &HeaderMap) -> AppResult<User> {
    let api_key = headers
        .get("x-api-key")
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let stored = user_repository::find_by_api_key(&state.db, api_key)
        .await?
        .ok_or(AppError::Unauthorized)?;

    Ok(stored.into())
}

fn build_auth_response(state: &AppState, user: User) -> AppResult<AuthResponse> {
    let token = create_token(&state.config.jwt_secret, user.id, &user.email)?;
    Ok(AuthResponse {
        api_key: user.api_key.clone(),
        user,
        token,
        expires_in_seconds: TOKEN_LIFETIME_SECONDS,
    })
}

fn create_token(secret: &str, user_id: Uuid, email: &str) -> AppResult<String> {
    let now = Utc::now();
    let claims = JwtClaims {
        sub: user_id.to_string(),
        email: email.to_string(),
        iat: now.timestamp() as usize,
        exp: (now + Duration::seconds(TOKEN_LIFETIME_SECONDS)).timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

fn decode_token(token: &str, secret: &str) -> AppResult<JwtClaims> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    let data = decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    Ok(data.claims)
}

fn generate_api_key() -> String {
    format!("ak_{}", Uuid::new_v4().simple())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_key_has_expected_prefix() {
        let key = generate_api_key();
        assert!(key.starts_with("ak_"));
        assert!(key.len() > 10);
    }
}
