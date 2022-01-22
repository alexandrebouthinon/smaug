mod types;

use crate::types::{Lock, State};
use actix_web::{
  delete, get, post, put, web::Json, App, HttpRequest, HttpResponse, HttpServer, Result,
};

static mut STATE: Option<State> = None;
static mut LOCK: Option<Lock> = None;

#[put("/lock")]
async fn lock(lock: Json<Lock>) -> Result<HttpResponse> {
  unsafe {
    if LOCK.is_some() {
      return Ok(HttpResponse::Conflict().json(&LOCK));
    }

    LOCK = Some(lock.into_inner());
    Ok(HttpResponse::Ok().json(&LOCK))
  }
}

#[delete("/lock")]
async fn unlock() -> Result<HttpResponse> {
  unsafe {
    LOCK = None;
  }
  Ok(HttpResponse::Ok().json(""))
}

#[get("/state")]
async fn get_state() -> Result<HttpResponse> {
  unsafe {
    if STATE.is_some() {
      return Ok(HttpResponse::Ok().json(&STATE));
    }
    Ok(HttpResponse::NotFound().json("{}"))
  }
}

#[post("/state")]
async fn set_state(req: HttpRequest, state: Json<State>) -> Result<HttpResponse> {
  let lock_id = req.query_string().split('=').collect::<Vec<&str>>()[1];

  unsafe {
    if LOCK.is_some() && LOCK.as_ref().unwrap().id != lock_id {
      return Ok(HttpResponse::Conflict().json(&LOCK));
    }
    LOCK = None;
    STATE = Some(state.clone());
    return Ok(HttpResponse::Ok().json(&STATE));
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(lock)
      .service(unlock)
      .service(get_state)
      .service(set_state)
    // .route("/hey", web::get().to(manual_hello))
  })
  .bind("127.0.0.1:8000")?
  .run()
  .await
}