use crate::models::user::UserWithPassword;
use chrono::Utc;
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub async fn create_user(
    db: &PgPool,
    email: &str,
    full_name: &str,
    password_hash: &str,
    api_key: &str,
) -> Result<UserWithPassword, sqlx::Error> {
    let user_id = Uuid::new_v4();
    let created_at = Utc::now();

    let row = sqlx::query(
        r#"
        INSERT INTO users (id, email, full_name, password_hash, api_key, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, email, full_name, password_hash, api_key, created_at
        "#,
    )
    .bind(user_id)
    .bind(email)
    .bind(full_name)
    .bind(password_hash)
    .bind(api_key)
    .bind(created_at)
    .fetch_one(db)
    .await?;

    Ok(UserWithPassword {
        id: row.try_get("id")?,
        email: row.try_get("email")?,
        full_name: row.try_get("full_name")?,
        password_hash: row.try_get("password_hash")?,
        api_key: row.try_get("api_key")?,
        created_at: row.try_get("created_at")?,
    })
}

pub async fn find_by_email(
    db: &PgPool,
    email: &str,
) -> Result<Option<UserWithPassword>, sqlx::Error> {
    let maybe_row = sqlx::query(
        r#"
        SELECT id, email, full_name, password_hash, api_key, created_at
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_optional(db)
    .await?;

    Ok(maybe_row.map(|row| UserWithPassword {
        id: row.try_get("id").expect("id"),
        email: row.try_get("email").expect("email"),
        full_name: row.try_get("full_name").expect("full_name"),
        password_hash: row.try_get("password_hash").expect("password_hash"),
        api_key: row.try_get("api_key").expect("api_key"),
        created_at: row.try_get("created_at").expect("created_at"),
    }))
}

pub async fn find_by_id(
    db: &PgPool,
    user_id: Uuid,
) -> Result<Option<UserWithPassword>, sqlx::Error> {
    let maybe_row = sqlx::query(
        r#"
        SELECT id, email, full_name, password_hash, api_key, created_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(db)
    .await?;

    Ok(maybe_row.map(|row| UserWithPassword {
        id: row.try_get("id").expect("id"),
        email: row.try_get("email").expect("email"),
        full_name: row.try_get("full_name").expect("full_name"),
        password_hash: row.try_get("password_hash").expect("password_hash"),
        api_key: row.try_get("api_key").expect("api_key"),
        created_at: row.try_get("created_at").expect("created_at"),
    }))
}

pub async fn find_by_api_key(
    db: &PgPool,
    api_key: &str,
) -> Result<Option<UserWithPassword>, sqlx::Error> {
    let maybe_row = sqlx::query(
        r#"
        SELECT id, email, full_name, password_hash, api_key, created_at
        FROM users
        WHERE api_key = $1
        "#,
    )
    .bind(api_key)
    .fetch_optional(db)
    .await?;

    Ok(maybe_row.map(|row| UserWithPassword {
        id: row.try_get("id").expect("id"),
        email: row.try_get("email").expect("email"),
        full_name: row.try_get("full_name").expect("full_name"),
        password_hash: row.try_get("password_hash").expect("password_hash"),
        api_key: row.try_get("api_key").expect("api_key"),
        created_at: row.try_get("created_at").expect("created_at"),
    }))
}
