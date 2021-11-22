mod domain;
mod infra;
mod routes;

use crate::infra::{oauth2, repositories};
use actix_cors::Cors;
use actix_web::{middleware, web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use tracing::info;
use tracing_subscriber;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "try_rust_solid=trace");
  std::env::set_var("RUST_BACKTRACE", "1");

  dotenv().ok();

  tracing_subscriber::fmt::init();

  let config = domain::Config::read().unwrap();

  info!("connecting to database");

  let db_pool = MySqlPoolOptions::new()
    .max_connections(10)
    .connect(&config.database.url)
    .await
    .unwrap();

  info!(
    "starting server at {}:{}",
    &config.server.host, &config.server.port
  );

  let address = (config.server.host.clone(), config.server.port);

  HttpServer::new(move || {
    App::new()
      .app_data(Data::new(domain::State {
        db: repositories::new(db_pool.clone()),
        config: config.clone(),
        oauth2_providers: oauth2::providers(),
      }))
      .wrap(middleware::Logger::default())
      .wrap(Cors::permissive())
      .configure(routes::init)
  })
  .bind(address)?
  .run()
  .await
}
