use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Project;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct User {
  pub(crate) id: Uuid,
  pub(crate) name: String,
  pub(crate) password: String,
  pub(crate) projects: Vec<Project>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_user_serialize() {
    let user = User {
      id: Uuid::new_v4(),
      name: "test".to_string(),
      password: "test".to_string(),
      projects: vec![],
    };
    let serialized = serde_json::to_string(&user).unwrap();
    let serialized_expected = format!(
      r#"{{"id":"{}","name":"test","password":"test","projects":[]}}"#,
      user.id
    );
    assert_eq!(serialized, serialized_expected);
  }
}
