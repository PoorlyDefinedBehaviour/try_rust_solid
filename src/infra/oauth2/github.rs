use crate::domain::{self, Config};
use anyhow::Result;
use async_trait::async_trait;
use tracing::instrument;

#[derive(Debug)]
pub struct Github;

#[async_trait]
impl domain::accounts::OAuth2Provider for Github {
  #[instrument(skip(self))]
  async fn exchange_code(&self, config: &Config, code: &str) -> Result<String> {
    todo!()
  }

  #[instrument(skip(self))]
  async fn fetch_user(
    &self,
    config: &Config,
    token: &str,
  ) -> Result<domain::accounts::dto::UpsertUser> {
    todo!()
  }
}
