use crate::domain;
use async_trait::async_trait;

#[derive(Debug)]
pub struct Github;

#[async_trait]
impl domain::auth::OAuth2Provider for Github {
  fn exchange_code(&self, code: String) -> Result<String> {
    todo!()
  }

  fn fetch_user(&self, token: String) -> Result<domain::accounts::dto::OAuth2User> {
    todo!()
  }
}
