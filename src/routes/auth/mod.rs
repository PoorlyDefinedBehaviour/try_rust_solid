use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::domain::{self, accounts};

pub fn init(config: &mut web::ServiceConfig) {
  config.service(oauth2_code_callback);
}

#[derive(Deserialize)]
struct OAuth2CallbackQuery {
  code: String,
}

#[derive(Deserialize)]
struct OAuth2CallbackParams {
  provider: String,
}

#[get("/auth/oauth2/{provider}")]
async fn oauth2_code_callback(
  state: web::Data<domain::State>,
  query: web::Query<OAuth2CallbackQuery>,
  params: web::Path<OAuth2CallbackParams>,
) -> impl Responder {
  match accounts::oauth2_signin(state.get_ref(), &params.provider, &query.code).await {
    Err(error) => HttpResponse::ServiceUnavailable().body(format!(
      "
      <h1> deu ruim </h1>
      <p> {} </p>
      ",
      error
    )),
    Ok(user) => HttpResponse::Ok().body(format!(
      r#"
      <p> logger in with {} </p>
      <p> {}@{} </p>
      <img src="{}">
    "#,
      user.oauth2_provider, user.name, user.email, user.profile_image_url,
    )),
  }
}
