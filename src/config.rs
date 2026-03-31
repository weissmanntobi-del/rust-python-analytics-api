use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_env: String,
    pub server_addr: String,
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub worker_buffer: usize,
    pub bcrypt_cost: u32,
    pub log_format: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            app_env: env_var("APP_ENV", "development"),
            server_addr: env_var("SERVER_ADDR", "0.0.0.0:8080"),
            database_url: env_required("DATABASE_URL")?,
            redis_url: env_required("REDIS_URL")?,
            jwt_secret: env_required("JWT_SECRET")?,
            worker_buffer: env_var("WORKER_BUFFER", "1024")
                .parse()
                .context("WORKER_BUFFER must be a positive integer")?,
            bcrypt_cost: env_var("BCRYPT_COST", "12")
                .parse()
                .context("BCRYPT_COST must be a positive integer")?,
            log_format: env_var("LOG_FORMAT", "pretty"),
        })
    }

    pub fn configure_tracing(&self) {
        let env_filter = std::env::var("RUST_LOG")
            .unwrap_or_else(|_| format!("info,{}=debug", env!("CARGO_PKG_NAME").replace('-', "_")));

        let builder = tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_target(false);

        if self.log_format == "json" {
            builder.json().init();
        } else {
            builder.compact().init();
        }
    }
}

fn env_required(name: &str) -> Result<String> {
    env::var(name).with_context(|| format!("{name} must be set"))
}

fn env_var(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_string())
}
