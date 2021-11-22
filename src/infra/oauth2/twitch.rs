use crate::domain::{self, Config};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Twitch;

#[derive(Debug, Serialize, Deserialize)]
struct ExchangeCodeResponse {
  access_token: String,
  refresh_token: String,
  expires_in: u128,
  scope: Vec<String>,
  token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FetchUserResponse {
  data: [User; 1],
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
  id: String,
  login: String,
  display_name: String,
  profile_image_url: String,
  email: String,
}

#[async_trait]
impl domain::accounts::OAuth2Provider for Twitch {
  async fn exchange_code(&self, config: &Config, code: &str) -> Result<String> {
    let response = reqwest::Client::new()
      .post(format!("{}/oauth2/token", config.twitch.auth_endpoint))
      .query(&[
        ("client_id", config.twitch.client_id.as_str()),
        ("client_secret", config.twitch.client_secret.as_str()),
        ("code", code),
        ("grant_type", "authorization_code"),
        (
          "redirect_uri",
          &format!("{}/auth/oauth2/twitch", &config.server.endpoint),
        ),
      ])
      .send()
      .await?
      .json::<ExchangeCodeResponse>()
      .await?;

    Ok(response.access_token)
  }

  async fn fetch_user(
    &self,
    config: &Config,
    token: &str,
  ) -> Result<domain::accounts::dto::UpsertUser> {
    let response = reqwest::Client::new()
      .get(format!("{}/users", &config.twitch.api_endpoint))
      .header("Client-ID", &config.twitch.client_id)
      .header("Authorization", &format!("Bearer {}", token))
      .header("Accept", "application/vnd.twitchtv.v5+json")
      .send()
      .await?
      .json::<FetchUserResponse>()
      .await?;

    let user = response.data[0].clone();

    Ok(domain::accounts::dto::UpsertUser {
      oauth2_provider_id: user.id,
      oauth2_provider: String::from("twitch"),
      name: user.display_name,
      email: user.email,
      profile_image_url: user.profile_image_url,
    })
  }
}
