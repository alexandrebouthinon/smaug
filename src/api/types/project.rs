use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Environment;
use super::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Project {
  pub(crate) id: Uuid,
  pub(crate) name: String,
  pub(crate) environments: Vec<Environment>,
  pub(crate) members: Vec<User>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_project_serialize() {
    let project = Project {
      id: Uuid::new_v4(),
      name: "test".to_string(),
      environments: vec![],
      members: vec![],
    };
    let serialized = serde_json::to_string(&project).unwrap();
    let serialized_expected = format!(
      r#"{{"id":"{}","name":"test","environments":[],"members":[]}}"#,
      project.id
    );

    assert_eq!(serialized, serialized_expected);
  }
}
