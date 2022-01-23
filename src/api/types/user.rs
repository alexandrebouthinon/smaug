use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Project;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct User {
  pub(crate) id: Uuid,
  pub(crate) name: String,
  pub(crate) email: String,
  pub(crate) password: String,
  pub(crate) projects: Vec<Project>,
}
