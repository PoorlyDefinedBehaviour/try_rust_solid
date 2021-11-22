use actix_web::{get, web, HttpResponse, Responder};

use crate::domain;

pub fn init(config: &mut web::ServiceConfig) {
  config.service(home);
}

#[get("/")]
async fn home(state: web::Data<domain::State>) -> impl Responder {
  HttpResponse::Ok().content_type("text/html").body(format!(
    r#"
    <div>
      <a href="{}">
        <p> Signin with Twitch <p>
      </a>
      <a>
        <p> Signin with Github <p>
      </a>
    </div>
    "#,
    format!(
      "{}?client_id={}&redirect_uri={}&response_type=code&scope=user:read:email",
      format!("{}/oauth2/authorize", state.config.twitch.auth_endpoint),
      state.config.twitch.client_id,
      format!("{}/auth/oauth2/twitch", state.config.server.endpoint)
    )
  ))
}
