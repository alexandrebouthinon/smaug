use super::{Lock, State};
use uuid::Uuid;

pub(crate) struct Environment {
  pub(crate) id: Uuid,
  pub(crate) name: String,
  pub(crate) lock: Option<Lock>,
  pub(crate) state: Option<State>,
  pub(crate) project_id: Uuid,
}

impl Environment {
  pub(crate) fn new(name: String, project_id: &Uuid) -> Self {
    Self {
      id: Uuid::new_v4(),
      lock: None,
      name,
      project_id: project_id.clone(),
      state: None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_environment_new() {
    let projet_id = Uuid::new_v4();
    let environment = Environment::new("test".to_string(), &projet_id);
    assert_ne!(environment.id.to_string(), "");
    assert_eq!(environment.name, "test");
    assert!(environment.lock.is_none());
    assert_eq!(environment.project_id, projet_id);
    assert!(environment.state.is_none());
  }
}
