use crate::domain;
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct Twitch;

#[async_trait]
impl domain::accounts::OAuth2Provider for Twitch {
  fn exchange_code(&self, code: String) -> Result<String> {
    todo!()
  }

  fn fetch_user(&self, token: String) -> Result<domain::accounts::dto::OAuth2User> {
    todo!()
  }
}
