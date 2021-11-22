#[derive(Debug)]
pub struct UpsertUser {
  pub oauth2_provider_id: String,
  pub oauth2_provider: String,
  pub name: String,
  pub email: String,
  pub profile_image_url: String,
}
