use actix_web::web;

mod auth;
mod home;

pub fn init(config: &mut web::ServiceConfig) {
  home::init(config);
  auth::init(config);
}
