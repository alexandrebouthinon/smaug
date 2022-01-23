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

impl Project {
  pub(crate) fn new(name: String) -> Self {
    Project {
      id: Uuid::new_v4(),
      name,
      environments: vec![],
      members: vec![],
    }
  }

  pub(crate) fn add_environment(&mut self, name: String) -> &mut Self {
    self.environments.push(Environment::new(name, &self.id));
    self
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_project_new() {
    let project = Project::new("test".to_string());
    assert_ne!(project.id.to_string(), "");
    assert_eq!(project.name, "test");
    assert_eq!(project.environments.len(), 0);
  }

  #[test]
  fn test_project_add_environment() {
    let mut project = Project::new("test".to_string());
    project.add_environment("test".to_string());
    assert_eq!(project.environments.len(), 1);
  }
}
