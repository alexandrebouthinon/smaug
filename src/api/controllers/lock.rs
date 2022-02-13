use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct Lock {
  #[serde(rename = "ID")]
  pub(crate) id: String,

  #[serde(rename = "Who")]
  pub who: String,

  #[serde(rename = "Created")]
  pub created: String,

  #[serde(rename = "Operation")]
  pub operation: String,

  #[serde(rename = "Info")]
  pub info: String,

  #[serde(rename = "Path")]
  pub path: String,

  #[serde(rename = "Version")]
  pub version: String,
}

use actix_web::{web, HttpResponse};

/// Create a lock using given information if it does not exist
pub(crate) fn lock(data: web::Data<crate::AppState>, lock: web::Json<Lock>) -> HttpResponse {
  let mut locks = data.locks.lock().unwrap();
  match locks.get(&lock.id) {
    Some(_) => HttpResponse::Conflict().finish(),
    None => {
      locks.insert(lock.id.clone(), lock.into_inner());
      HttpResponse::Ok().finish()
    }
  }
}

/// Delete a lock using the given ID
pub(crate) fn unlock(data: web::Data<crate::AppState>, lock: web::Json<Lock>) -> HttpResponse {
  let mut locks = data.locks.lock().unwrap();
  match locks.remove(&lock.id) {
    Some(_) => HttpResponse::Ok().finish(),
    None => HttpResponse::NotFound().finish(),
  }
}

use actix_web::http::Method;

/// Generate routes for lock controller
pub(crate) fn routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("/")
      .route(web::method(Method::from_bytes(b"LOCK").unwrap()).to(self::lock))
      .route(web::method(Method::from_bytes(b"UNLOCK").unwrap()).to(self::unlock)),
  );
}

/// Test for the lock controller
///
/// # Modules
/// - `lock`: Test for the Lock struct
/// - `routes`: Test for the routes
#[cfg(test)]
mod tests {
  use super::*;

  #[cfg(test)]
  mod lock {
    use super::*;
    use std::error::Error;

    #[test]
    fn serialize() -> Result<(), Box<dyn Error>> {
      let lock = Lock {
        id: "1".to_string(),
        who: "test".to_string(),
        created: "test".to_string(),
        operation: "test".to_string(),
        info: "test".to_string(),
        path: "test".to_string(),
        version: "test".to_string(),
      };
      let serialized = serde_json::to_string(&lock)?;
      let serialized_expected = r#"{"ID":"1","Who":"test","Created":"test","Operation":"test","Info":"test","Path":"test","Version":"test"}"#;
      assert_eq!(serialized, serialized_expected);
      Ok(())
    }

    #[test]
    fn deserialize() -> Result<(), Box<dyn Error>> {
      let lock = serde_json::from_str::<Lock>(&String::from(
        r#"{
          "ID": "d5b9f8f8-c9f8-4f8f-b8f8-f8f8f8f8f8f8",
          "Who": "terraform",
          "Created": "2020-04-01T00:00:00Z",
          "Operation": "plan",
          "Info": "",
          "Path": "./",
          "Version": "0.12.0"
        }"#,
      ))?;

      assert_eq!(lock.id.as_str(), "d5b9f8f8-c9f8-4f8f-b8f8-f8f8f8f8f8f8");
      assert_eq!(lock.who, "terraform");
      assert_eq!(lock.created, "2020-04-01T00:00:00Z");
      assert_eq!(lock.operation, "plan");
      assert_eq!(lock.info, "");
      assert_eq!(lock.path, "./");
      assert_eq!(lock.version, "0.12.0");

      Ok(())
    }
  }

  mod routes {
    use super::*;
    use crate::api::AppState;

    use actix_web::{test, App};
    use serde_json::json;

    mod lock {
      use super::*;

      #[actix_rt::test]
      async fn lock_once() {
        let mut app = test::init_service(App::new().data(AppState::new()).configure(routes)).await;
        let req = test::TestRequest::with_uri("/")
          .method(Method::from_bytes(b"LOCK").unwrap())
          .set_json(&json!({
            "ID": "d5b9f8f8-c9f8-4f8f-b8f8-f8f8f8f8f8f8",
            "Who": "terraform",
            "Created": "2020-04-01T00:00:00Z",
            "Operation": "plan",
            "Info": "",
            "Path": "./",
            "Version": "0.12.0"
          }))
          .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
      }

      #[actix_rt::test]
      async fn lock_twice_with_same_id() {
        let mut app = test::init_service(App::new().data(AppState::new()).configure(routes)).await;
        let req = test::TestRequest::with_uri("/")
          .method(Method::from_bytes(b"LOCK").unwrap())
          .set_json(&json!({
            "ID": "d5b9f8f8-c9f8-4f8f-b8f8-f8f8f8f8f8f8",
            "Who": "terraform",
            "Created": "2020-04-01T00:00:00Z",
            "Operation": "plan",
            "Info": "",
            "Path": "./",
            "Version": "0.12.0"
          }))
          .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);

        let req = test::TestRequest::with_uri("/")
          .method(Method::from_bytes(b"LOCK").unwrap())
          .set_json(&json!({
            "ID": "d5b9f8f8-c9f8-4f8f-b8f8-f8f8f8f8f8f8",
            "Who": "terraform",
            "Created": "2020-04-01T00:00:00Z",
            "Operation": "plan",
            "Info": "",
            "Path": "./",
            "Version": "0.12.0"
          }))
          .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 409);
      }

      #[actix_rt::test]
      async fn lock_twice_with_different_id() {
        let mut app = test::init_service(App::new().data(AppState::new()).configure(routes)).await;
        let req = test::TestRequest::with_uri("/")
          .method(Method::from_bytes(b"LOCK").unwrap())
          .set_json(&json!({
            "ID": "d5b9f8f8-c9f8-4f8f-b8f8-f8f8f8f8f8f8",
            "Who": "terraform",
            "Created": "2020-04-01T00:00:00Z",
            "Operation": "plan",
            "Info": "",
            "Path": "./",
            "Version": "0.12.0"
          }))
          .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);

        let req = test::TestRequest::with_uri("/")
          .method(Method::from_bytes(b"LOCK").unwrap())
          .set_json(&json!({
            "ID": "d5b9f8f8-c9f8-4f8f-b8f8-fd43242552f2",
            "Who": "terraform",
            "Created": "2020-04-01T00:00:00Z",
            "Operation": "plan",
            "Info": "",
            "Path": "./",
            "Version": "0.12.0"
          }))
          .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
      }
    }

    mod unlock {
      use super::*;

      #[actix_rt::test]
      async fn unlock_not_existing() {
        let mut app = test::init_service(App::new().data(AppState::new()).configure(routes)).await;
        let req = test::TestRequest::with_uri("/")
          .method(Method::from_bytes(b"UNLOCK").unwrap())
          .set_json(&json!({
            "ID": "d5b9f8f8-c9f8-4f8f-b8f8-f8f8f8f8f8f8",
            "Who": "terraform",
            "Created": "2020-04-01T00:00:00Z",
            "Operation": "plan",
            "Info": "",
            "Path": "./",
            "Version": "0.12.0"
          }))
          .to_request();

        dbg!(&req);

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 404);
      }

      #[actix_rt::test]
      async fn unlock_existing() {
        let mut app = test::init_service(App::new().data(AppState::new()).configure(routes)).await;
        let req = test::TestRequest::with_uri("/")
          .method(Method::from_bytes(b"LOCK").unwrap())
          .set_json(&json!({
            "ID": "d5b9f8f8-c9f8-4f8f-b8f8-f8f8f8f8f8f8",
            "Who": "terraform",
            "Created": "2020-04-01T00:00:00Z",
            "Operation": "plan",
            "Info": "",
            "Path": "./",
            "Version": "0.12.0"
          }))
          .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);

        let req = test::TestRequest::with_uri("/")
          .method(Method::from_bytes(b"UNLOCK").unwrap())
          .set_json(&json!({
            "ID": "d5b9f8f8-c9f8-4f8f-b8f8-f8f8f8f8f8f8",
            "Who": "terraform",
            "Created": "2020-04-01T00:00:00Z",
            "Operation": "plan",
            "Info": "",
            "Path": "./",
            "Version": "0.12.0"
          }))
          .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
      }
    }
  }
}
