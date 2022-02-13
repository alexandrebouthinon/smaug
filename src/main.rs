mod api;

use actix_web::{
  middleware::{self, normalize::TrailingSlash},
  App, HttpServer,
};
use api::State;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
  env_logger::init();

  HttpServer::new(|| {
    let app = App::new()
      .data(State::new())
      .configure(api::controllers::lock::routes)
      .wrap(middleware::Logger::default())
      .wrap(middleware::NormalizePath::new(TrailingSlash::Always));
    app
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
