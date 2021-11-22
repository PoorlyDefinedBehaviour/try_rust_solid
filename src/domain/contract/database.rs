use anyhow::Result;
use async_trait::async_trait;

use crate::domain::accounts;

pub struct Database {
  pub users: Box<dyn UserRepository>,
}

#[async_trait]
pub trait UserRepository {
  async fn upsert(&self, data: accounts::dto::UpsertUser) -> Result<accounts::User>;
  async fn get_by_email(&self, email: &str) -> Result<Option<accounts::User>>;
}
