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

#[cfg(test)]
mod tests {
  use serde_json::{Map, Value::Object};
  use std::error::Error;

  use super::*;

  #[test]
  fn test_state_new() -> Result<(), Box<dyn Error>> {
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
}
