// src/config.rs
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub bot_token: String,

    pub ch_host: String,
    pub native_port: u16,
    pub ch_secure: bool,
    pub ch_insecure_skip_verify: bool,
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
        let native_port = std::env::var("CH_NATIVE_PORT")
            .or_else(|_| std::env::var("CH_PORT"))
            .unwrap_or_else(|_| "9000".into())
            .parse()?;
        let ch_secure = std::env::var("CH_SECURE")
            .unwrap_or_else(|_| "false".into())
            .parse()?;
        let ch_insecure_skip_verify = std::env::var("CH_INSECURE_SKIP_VERIFY")
            .unwrap_or_else(|_| "false".into())
            .parse()?;
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
            native_port,
            ch_secure,
            ch_insecure_skip_verify,
            ch_user,
            ch_password,
            ch_database,
            blocked_file,
            db_queue_maxsize,
            query_timeout,
        })
    }

    pub fn ch_dsn(&self) -> String {
        let proto = if self.ch_secure { "tcp+tls" } else { "tcp" };
        let mut dsn = format!(
            "{}://{}:{}@{}:{}/{}",
            proto, self.ch_user, self.ch_password, self.ch_host, self.native_port, self.ch_database
        );

        if self.ch_secure && self.ch_insecure_skip_verify {
            dsn.push_str("?skip_verify=true");
        }

        dsn
    }
}
