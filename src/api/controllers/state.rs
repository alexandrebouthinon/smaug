use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct State {
  lineage: String,
  outputs: Value,
  resources: Value,
  serial: u64,
  terraform_version: String,
  version: u64,
}

use actix_web::{web, HttpResponse};

/// Extractor for lock id
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct LockArguments {
  #[serde(rename = "ID")]
  pub id: String,
}

/// Returns the Terraform state for the given workspace
pub(crate) fn get_state(
  data: web::Data<crate::AppState>,
  web::Path(name): web::Path<String>,
) -> HttpResponse {
  let states = data.states.lock().unwrap();
  match states.get(&name) {
    Some(state) => HttpResponse::Ok().json(state),
    None => HttpResponse::NotFound().finish(),
  }
}

/// Create or replace a Terraform state for the given workspace
pub(crate) fn create_update_state(
  data: web::Data<crate::AppState>,
  web::Path(state): web::Path<String>,
  params: web::Query<LockArguments>,
  update: web::Json<State>,
) -> HttpResponse {
  let mut states = data.states.lock().unwrap();
  let locks = data.locks.lock().unwrap();
  match locks.get(&params.id) {
    Some(_) => {
      states.insert(state, update.into_inner());
      HttpResponse::Ok().finish()
    }
    None => HttpResponse::NotFound().finish(),
  }
}

pub(crate) fn routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("/{name}")
      .route(web::get().to(self::get_state))
      .route(web::post().to(self::create_update_state)),
  );
}

#[cfg(test)]
mod tests {
  use super::*;

  mod state {
    use super::*;
    use serde_json::{Map, Value::Object};
    use std::error::Error;

    #[test]
    fn deserialize() -> Result<(), Box<dyn Error>> {
      let state = serde_json::from_str::<State>(&String::from(
        r#"{
          "lineage": "",
          "outputs": {},
          "resources": {},
          "serial": 0,
          "terraform_version": "0.12.0",
          "version": 0
        }"#,
      ))?;

      assert_eq!(state.lineage, "");
      assert_eq!(state.outputs, Object(Map::new()));
      assert_eq!(state.resources, Object(Map::new()));
      assert_eq!(state.serial, 0);
      assert_eq!(state.terraform_version, "0.12.0");
      assert_eq!(state.version, 0);

      Ok(())
    }

    #[test]
    fn serialize() -> Result<(), Box<dyn Error>> {
      let state = State {
        lineage: "".to_string(),
        outputs: serde_json::from_str::<Value>(r#"{}"#)?,
        resources: serde_json::from_str::<Value>(r#"{}"#)?,
        serial: 0,
        terraform_version: "0.12.0".to_string(),
        version: 0,
      };

      let serialized = serde_json::to_string(&state)?;

      assert_eq!(
        serialized,
        r#"{"lineage":"","outputs":{},"resources":{},"serial":0,"terraform_version":"0.12.0","version":0}"#
      );

      Ok(())
    }
  }

  mod routes {
    use super::*;
    use crate::api::controllers::lock::Lock;
    use crate::api::AppState;

    use actix_web::{test, App};
    use serde_json::json;

    mod get {
      use super::*;

      #[actix_rt::test]
      async fn state_not_found() {
        let mut app = test::init_service(App::new().data(AppState::new()).configure(routes)).await;
        let req = test::TestRequest::get().uri("/test").to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 404);
      }

      #[actix_rt::test]
      async fn state_found() {
        let app_state = AppState::new();
        app_state.locks.lock().unwrap().insert(
          "1234".to_string(),
          Lock {
            id: "1234".to_string(),
            who: "test".to_string(),
            created: "test".to_string(),
            operation: "test".to_string(),
            info: "test".to_string(),
            path: "test".to_string(),
            version: "test".to_string(),
          },
        );

        let mut app = test::init_service(App::new().data(app_state).configure(routes)).await;
        let req = test::TestRequest::post()
          .uri("/test?ID=1234")
          .set_json(&State {
            lineage: "".to_string(),
            outputs: json!({}),
            resources: json!({}),
            serial: 0,
            terraform_version: "0.12.0".to_string(),
            version: 0,
          })
          .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);

        let req = test::TestRequest::get().uri("/test").to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
      }
    }

    mod post {
      use super::*;

      #[actix_rt::test]
      async fn create() {
        let app_state = AppState::new();
        app_state.locks.lock().unwrap().insert(
          "1234".to_string(),
          Lock {
            id: "1234".to_string(),
            who: "test".to_string(),
            created: "test".to_string(),
            operation: "test".to_string(),
            info: "test".to_string(),
            path: "test".to_string(),
            version: "test".to_string(),
          },
        );

        let mut app = test::init_service(App::new().data(app_state).configure(routes)).await;
        let req = test::TestRequest::post()
          .uri("/test?ID=1234")
          .set_json(&State {
            lineage: "".to_string(),
            outputs: json!({}),
            resources: json!({}),
            serial: 0,
            terraform_version: "0.12.0".to_string(),
            version: 0,
          })
          .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
      }

      #[actix_rt::test]
      async fn missing_lock() {
        let mut app = test::init_service(App::new().data(AppState::new()).configure(routes)).await;
        let req = test::TestRequest::post()
          .uri("/test?ID=1234")
          .set_json(&State {
            lineage: "".to_string(),
            outputs: json!({}),
            resources: json!({}),
            serial: 0,
            terraform_version: "0.12.0".to_string(),
            version: 0,
          })
          .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 404);
      }
    }
  }
}
