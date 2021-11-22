use anyhow::Result;
use async_trait::async_trait;
use sqlx::{mysql::MySqlRow, MySql, Pool, Row};
use tracing::instrument;

use crate::domain::{accounts, contract};

pub(super) struct UserRepository {
  pub pool: Pool<MySql>,
}

fn row_to_user(row: MySqlRow) -> Result<accounts::User, sqlx::Error> {
  Ok(accounts::User {
    id: row.try_get("user_id")?,
    name: row.try_get("user_name")?,
    email: row.try_get("user_email")?,
    profile_image_url: row.try_get("user_profile_image_url")?,
    oauth2_provider_id: row.try_get("user_oauth2_provider_id")?,
    oauth2_provider: row.try_get("user_oauth2_provider")?,
  })
}

#[async_trait]
impl contract::database::UserRepository for UserRepository {
  #[instrument(skip(self))]
  async fn upsert(&self, data: accounts::dto::UpsertUser) -> Result<accounts::User> {
    sqlx::query!(
      "
      INSERT INTO tab_user (
        name, 
        email, 
        profile_image_url, 
        oauth2_provider_id,
        oauth2_provider
      )
      VALUES (?, ?, ?, ?, ?)
      ON DUPLICATE KEY UPDATE
      name = VALUES(name),
      profile_image_url = VALUES(profile_image_url),
      oauth2_provider_id = VALUES(oauth2_provider_id),
      oauth2_provider = VALUES(oauth2_provider)
      ",
      data.name,
      data.email,
      data.profile_image_url,
      data.oauth2_provider_id,
      data.oauth2_provider,
    )
    .execute(&self.pool)
    .await?;

    let user = self.get_by_email(&data.email).await?;

    Ok(user.unwrap())
  }

  async fn get_by_email(&self, email: &str) -> Result<Option<accounts::User>> {
    let row = sqlx::query(
      "
      SELECT 
        id as user_id, 
        name as user_name,
        email as user_email,
        profile_image_url as user_profile_image_url,
        oauth2_provider_id as user_oauth2_provider_id,
        oauth2_provider as user_oauth2_provider
      FROM tab_user
      WHERE email = ?
      ",
    )
    .bind(email)
    .fetch_optional(&self.pool)
    .await?;

    match row {
      None => Ok(None),
      Some(row) => Ok(Some(row_to_user(row)?)),
    }
  }
}
