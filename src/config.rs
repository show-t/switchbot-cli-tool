use anyhow::Result;
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub token: String,
    pub secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();

        let host = std::env::var("SWITCHBOT_API_HOST")?;
        let token = std::env::var("SWITCHBOT_TOKEN")?;
        let secret = std::env::var("SWITCHBOT_SECRET")?;

        Ok(Self {
            host,
            token,
            secret,
        })
    }
}
