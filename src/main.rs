mod api;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(Env::default().default_filter_or("info"));

  HttpServer::new(|| App::new().wrap(Logger::default()))
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
