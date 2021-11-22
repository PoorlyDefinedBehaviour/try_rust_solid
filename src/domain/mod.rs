use std::collections::HashMap;

pub mod accounts;
pub mod config;
pub mod contract;

pub use config::Config;

use contract::database::Database;

pub struct DI {
  pub config: Config,
  pub db: Database,
  pub oauth2_providers: HashMap<String, Box<dyn accounts::OAuth2Provider>>,
}
