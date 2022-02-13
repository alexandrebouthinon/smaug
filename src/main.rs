mod api;
use api::AppState;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
  env_logger::init();

  HttpServer::new(|| {
    let app = App::new()
      .data(AppState::new())
      .configure(api::controllers::lock::routes)
      .configure(api::controllers::state::routes)
      .wrap(Logger::default());
    app
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
