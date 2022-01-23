use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct Lock {
  #[serde(rename = "ID")]
  pub id: String,

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

#[cfg(test)]
mod tests {
  use super::*;
  use std::error::Error;

  #[test]
  fn test_lock_new() -> Result<(), Box<dyn Error>> {
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
