mod github;
mod twitch;

use std::collections::HashMap;

pub use github::Github;
pub use twitch::Twitch;

use crate::domain::accounts;

pub fn providers() -> HashMap<String, Box<dyn accounts::OAuth2Provider>> {
  let mut oauth2_providers: HashMap<String, Box<dyn accounts::OAuth2Provider>> = HashMap::new();

  oauth2_providers.insert(String::from("github"), Box::new(Github));
  oauth2_providers.insert(String::from("twitch"), Box::new(Twitch));

  oauth2_providers
}
