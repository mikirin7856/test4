// src/config.rs
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub bot_token: String,

    pub ch_host: String,
    pub ch_port: u16,
    pub ch_secure: bool,
    pub ch_compression: bool,
    pub ch_user: String,
    pub ch_password: String,
    pub ch_database: String,

    pub blocked_file: String,

    pub db_queue_maxsize: usize,
    pub query_timeout: u64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let bot_token = std::env::var("BOT_TOKEN").context("test")?;

        let ch_host = std::env::var("CH_HOST").unwrap_or_else(|_| "localhost".into());
        let ch_port = std::env::var("CH_PORT")
            .unwrap_or_else(|_| "9000".into())
            .parse()?;
        let ch_secure = std::env::var("CH_SECURE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let ch_compression = std::env::var("CH_COMPRESSION")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(true);
        let ch_user = std::env::var("CH_USER").unwrap_or_else(|_| "default".into());
        let ch_password = std::env::var("CH_PASSWORD").unwrap_or_else(|_| "test".into());
        let ch_database = std::env::var("CH_DATABASE").unwrap_or_else(|_| "default".into());

        let blocked_file = std::env::var("BLOCKED_FILE").unwrap_or_else(|_| "blocked.txt".into());
        let db_queue_maxsize = std::env::var("DB_QUEUE_MAXSIZE")
            .unwrap_or_else(|_| "100".into())
            .parse()?;
        let query_timeout = std::env::var("QUERY_TIMEOUT")
            .unwrap_or_else(|_| "30".into())
            .parse()?;

        Ok(Self {
            bot_token,
            ch_host,
            ch_port,
            ch_secure,
            ch_compression,
            ch_user,
            ch_password,
            ch_database,
            blocked_file,
            db_queue_maxsize,
            query_timeout,
        })
    }
}
