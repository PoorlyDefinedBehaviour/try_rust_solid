use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct TwitchConfig {
  pub auth_endpoint: String,
  pub api_endpoint: String,
  pub client_id: String,
  pub client_secret: String,
}

#[derive(Debug, Clone)]
pub struct GithubConfig {
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
  pub github: GithubConfig,
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
      github: GithubConfig {
        auth_endpoint: env("GITHUB_AUTH_ENDPOINT")?,
        api_endpoint: env("GITHUB_API_ENDPOINT")?,
        client_id: env("GITHUB_CLIENT_ID")?,
        client_secret: env("GITHUB_CLIENT_SECRET")?,
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
