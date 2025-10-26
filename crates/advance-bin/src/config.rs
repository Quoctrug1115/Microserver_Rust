use serde::Deserialize;
use sqlx::Postgres;
use crate::err::AppError;

#[derive(Deserialize)]
pub struct WebConfig {
    pub addr: String,
}

#[derive(Deserialize)]
pub struct PostgresSQL {
    pub dsn: String,
    pub conn : i32,
}

#[derive(Deserialize)]
pub struct Config {
    pub web: WebConfig,
    pub postgres: PostgresSQL,
}

impl Config {
    pub fn from_env() -> Result<Config> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .map_err(AppError::from)?
            .try_deserialize() // giai ma tra ve Result nen map_err 2 lan
            .map_err(AppError::from)
    }
}