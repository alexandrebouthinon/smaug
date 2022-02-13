use super::controllers::{lock::Lock, state::State};
use std::{collections::HashMap, sync::Mutex};

pub(crate) struct AppState {
  pub(crate) locks: Mutex<HashMap<String, Lock>>,
  pub(crate) states: Mutex<HashMap<String, State>>,
}

impl AppState {
  pub(crate) fn new() -> Self {
    AppState {
      locks: Mutex::new(HashMap::new()),
      states: Mutex::new(HashMap::new()),
    }
  }
}
