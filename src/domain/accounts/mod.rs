use crate::domain;
use anyhow::{bail, Result};
use async_trait::async_trait;
use thiserror::Error;

use super::Config;

pub mod dto;

#[derive(Debug)]
pub struct User {
  pub id: u64,
  pub name: String,
  pub email: String,
  pub profile_image_url: String,
  pub oauth2_provider_id: String,
  pub oauth2_provider: String,
}

#[async_trait]
pub trait OAuth2Provider {
  async fn exchange_code(&self, config: &Config, code: &str) -> Result<String>;
  async fn fetch_user(&self, config: &Config, token: &str) -> Result<dto::UpsertUser>;
}

#[derive(Debug, PartialEq, Error)]
pub enum OAuth2SigningError {
  #[error("expected signin with {expected:?}, got {got:?}")]
  UnexpectedProvider { expected: String, got: String },
}

pub async fn oauth2_signin(di: &domain::DI, provider_name: &str, code: &str) -> Result<User> {
  match di.oauth2_providers.get(provider_name) {
    None => bail!("unknown provider: {}", provider_name),
    Some(provider) => {
      let access_token = provider.exchange_code(&di.config, code).await?;

      let provider_user = provider.fetch_user(&di.config, &access_token).await?;

      match di.db.users.get_by_email(&provider_user.email).await? {
        None => di.db.users.upsert(provider_user).await,
        Some(user) => {
          if user.oauth2_provider != provider_name {
            return Err(
              OAuth2SigningError::UnexpectedProvider {
                expected: user.oauth2_provider.clone(),
                got: String::from(provider_name),
              }
              .into(),
            );
          }

          di.db.users.upsert(provider_user).await
        }
      }
    }
  }
}
