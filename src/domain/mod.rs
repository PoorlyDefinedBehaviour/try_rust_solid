use std::collections::HashMap;

use anyhow::{bail, Result};

use self::contract::database::Database;

pub mod accounts;
pub mod contract;

#[derive(Debug, Clone)]
pub struct TwitchConfig {
  pub auth_endpoint: String,
  pub api_endpoint: String,
  pub client_id: String,
  pub client_secret: String,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
  pub endpoint: String,
  pub host: String,
  pub port: u16,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
  pub url: String,
}

#[derive(Debug, Clone)]
pub struct Config {
  pub server: ServerConfig,
  pub twitch: TwitchConfig,
  pub database: DatabaseConfig,
}

fn env(key: &str) -> Result<String> {
  match std::env::var(key) {
    Err(_) => bail!("environment variable not found: {}", key),
    Ok(value) => Ok(value),
  }
}

impl Config {
  pub fn read() -> Result<Config> {
    Ok(Config {
      twitch: TwitchConfig {
        api_endpoint: env("TWITCH_API_ENDPOINT")?,
        auth_endpoint: env("TWITCH_AUTH_ENDPOINT")?,
        client_id: env("TWITCH_CLIENT_ID")?,
        client_secret: env("TWITCH_CLIENT_SECRET")?,
      },
      server: ServerConfig {
        endpoint: env("ENDPOINT")?,
        host: env("HOST")?,
        port: env("PORT").and_then(|s| s.parse::<u16>().map_err(anyhow::Error::msg))?,
      },
      database: DatabaseConfig {
        url: env("DATABASE_URL")?,
      },
    })
  }
}

pub struct State {
  pub config: Config,
  pub db: Database,
  pub oauth2_providers: HashMap<String, Box<dyn accounts::OAuth2Provider>>,
}
