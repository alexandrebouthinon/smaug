use super::{Lock, State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Environment {
  pub(crate) id: Uuid,
  pub(crate) name: String,
  pub(crate) lock: Option<Lock>,
  pub(crate) state: Option<State>,
  pub(crate) project_id: Uuid,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_environment_serialize() {
    let environment = Environment {
      id: Uuid::new_v4(),
      name: "test".to_string(),
      lock: None,
      state: None,
      project_id: Uuid::new_v4(),
    };
    let serialized = serde_json::to_string(&environment).unwrap();
    let serialized_expected = format!(
      r#"{{"id":"{}","name":"test","lock":null,"state":null,"project_id":"{}"}}"#,
      environment.id, environment.project_id
    );
    assert_eq!(serialized, serialized_expected);
  }
}
