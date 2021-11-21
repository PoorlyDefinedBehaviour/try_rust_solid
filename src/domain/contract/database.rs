use anyhow::Result;
use async_trait::async_trait;

use crate::domain::accounts;

pub struct Database {}

#[async_trait]
pub trait UserRepository {
  fn upsert(&self, data: accounts::dto::OAuth2User) -> Result<accounts::User>;
}
