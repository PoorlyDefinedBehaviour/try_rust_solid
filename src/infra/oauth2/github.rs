use crate::domain::{self, Config};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use tracing::instrument;
use urlencoding;

#[derive(Debug)]
pub struct Github;

#[derive(Deserialize)]
struct ExchangeCodeResponse {
  access_token: String,
}

#[derive(Deserialize)]
struct FetchUserResponse {
  id: u64,
  avatar_url: String,
  email: String,
  name: String,
}

#[async_trait]
impl domain::accounts::OAuth2Provider for Github {
  #[instrument(skip(self))]
  async fn exchange_code(&self, config: &Config, code: &str) -> Result<String> {
    let response = reqwest::Client::new()
      .post(format!(
        "{}/login/oauth/access_token",
        config.github.auth_endpoint
      ))
      .header("accept", "application/json")
      .query(&[
        ("client_id", config.github.client_id.as_str()),
        ("client_secret", config.github.client_secret.as_str()),
        ("code", code),
        (
          "redirect_uri",
          &format!("{}/auth/oauth2/github", &config.server.endpoint),
        ),
      ])
      .send()
      .await?
      .json::<ExchangeCodeResponse>()
      .await?;

    Ok(response.access_token)
  }

  #[instrument(skip(self))]
  async fn fetch_user(
    &self,
    config: &Config,
    token: &str,
  ) -> Result<domain::accounts::dto::UpsertUser> {
    let response = reqwest::Client::new()
      .get(format!("{}/user", &config.github.api_endpoint))
      .header(
        "user-agent",
        urlencoding::encode(
          "
        WWWWWW||WWWWWW
         W W W||W W W
              ||
            ( OO )__________
            |   |           |
            |o o|    MIT     |
            |___|||_||__||_|| *
                 || ||  || ||
                _||_|| _||_||
               (__|__|(__|__|
      ",
        )
        .to_string(),
      )
      .header("Authorization", &format!("Bearer {}", token))
      .header("Accept", "application/json")
      .send()
      .await?
      .json::<FetchUserResponse>()
      .await?;

    Ok(domain::accounts::dto::UpsertUser {
      oauth2_provider_id: response.id.to_string(),
      oauth2_provider: String::from("github"),
      name: response.name,
      email: response.email,
      profile_image_url: response.avatar_url,
    })
  }
}
