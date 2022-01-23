use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct Lock {
  #[serde(rename = "ID")]
  pub(crate) id: String,

  #[serde(rename = "Who")]
  who: String,

  #[serde(rename = "Created")]
  created: String,

  #[serde(rename = "Operation")]
  operation: String,

  #[serde(rename = "Info")]
  info: String,

  #[serde(rename = "Path")]
  path: String,

  #[serde(rename = "Version")]
  version: String,
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::error::Error;

  #[test]
  fn test_lock_deserialize() -> Result<(), Box<dyn Error>> {
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
