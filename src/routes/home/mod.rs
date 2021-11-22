use actix_web::{get, web, HttpResponse, Responder};

use crate::domain;

pub fn init(config: &mut web::ServiceConfig) {
  config.service(home);
}

#[get("/")]
async fn home(di: web::Data<domain::DI>) -> impl Responder {
  HttpResponse::Ok().content_type("text/html").body(format!(
    r#"
    <div>
      <a href="{}">
        <p> Signin with Twitch <p>
      </a>
      <a href="{}">
        <p> Signin with Github <p>
      </a>
    </div>
    "#,
    format!(
      "{}/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=user:read:email",
      di.config.twitch.auth_endpoint,
      di.config.twitch.client_id,
      format!("{}/auth/oauth2/twitch", di.config.server.endpoint)
    ),
    format!(
      "{}/login/oauth/authorize?client_id={}&redirect_uri={}&scope=user:email",
      di.config.github.auth_endpoint,
      di.config.github.client_id,
      format!("{}/auth/oauth2/github", di.config.server.endpoint)
    ),
  ))
}
