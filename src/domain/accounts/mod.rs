use anyhow::Result;
use async_trait::async_trait;

pub mod dto;

#[derive(Debug)]
pub struct User {}

#[async_trait]
pub trait OAuth2Provider {
  fn exchange_code(&self, code: String) -> Result<String>;
  fn fetch_user(&self, token: String) -> Result<dto::OAuth2User>;
}
