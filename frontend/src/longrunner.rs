//! Client-side LongRunner tools
//!
//! These are tools that map messages into the frontend components. This is primarily a context for
//! routing received emissions to the places that can make use of them.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use tracing::info;
use uuid::Uuid;
use yew::Callback;

/// A pub/sub system for the client side of the LongRunner. This handles much of the boilerplate for listening
#[derive(Debug, Clone)]
pub struct LongRunnerCtx {
  /// A routing table to map messages for each process id to all the reducers that care about that
  /// specific request id
  forwards: Arc<RwLock<HashMap<Uuid, Vec<Uuid>>>>,

  /// A map of component id to its reducer/dispatch
  reducers: Arc<RwLock<HashMap<Uuid, Callback<String>>>>,
}

impl LongRunnerCtx {
  pub fn new() -> LongRunnerCtx {
    LongRunnerCtx {
      forwards: Arc::new(RwLock::new(HashMap::new())),
      reducers: Arc::new(RwLock::new(HashMap::new())),
    }
  }

  // Set
  pub fn register(&self, guid: Uuid, reducer: Callback<String>) -> Uuid {
    info!("Registering a new component: {:?}", guid);
    let mut reducers = self.reducers.write().unwrap();
    match reducers.insert(guid, reducer) {
      Some(_) => info!("Updated the reducer for {}", guid),
      None => info!("Inserted the reducer for new guid {}", guid),
    }
    info!("There are {} reducers", reducers.keys().len());
    guid
  }

  pub fn update(&self, _component_id: Uuid, _reducer: String) {}

  pub fn deregister(&self, _component_id: Uuid) {}

  pub fn subscribe(&self, _proc_id: Uuid, _component_id: Uuid) {}

  pub fn unsubscribe(&self, _proc_id: Uuid, _component_id: Uuid) {}

  pub fn send(&self) -> Uuid {
    Uuid::new_v4()
  }

  pub fn receive(&self) {}
}

// This is essentially static with internal mutability, so the equality should always be true
impl PartialEq for LongRunnerCtx {
  fn eq(&self, _other: &Self) -> bool {
    true
  }
}
