use super::controllers::lock::Lock;
use std::{collections::HashMap, sync::Mutex};

pub(crate) struct State {
  pub(crate) locks: Mutex<HashMap<String, Lock>>,
}

impl State {
  pub(crate) fn new() -> Self {
    State {
      locks: Mutex::new(HashMap::new()),
    }
  }
}
