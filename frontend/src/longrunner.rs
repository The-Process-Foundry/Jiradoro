//! Client-side LongRunner tools
//!
//! These are tools that map messages into the frontend components. This is primarily a context for
//! routing received emissions to the places that can make use of them.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use tracing::info;
use uuid::Uuid;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
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

// This is essentially static with internal mutability, so the equality should always be true
impl PartialEq for LongRunnerCtx {
  fn eq(&self, _other: &Self) -> bool {
    true
  }
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
    let mut reducers = self.reducers.write().unwrap();
    match reducers.insert(guid, reducer) {
      Some(_) => info!("Updated the reducer for {}", guid),
      None => info!("Inserted the reducer for new guid {}", guid),
    }
    guid
  }

  pub fn update(&self, _component_id: Uuid, _reducer: String) {}

  pub fn deregister(&self, _component_id: Uuid) {}

  pub fn subscribe(&self, _proc_id: Uuid, _component_id: Uuid) {}

  pub fn unsubscribe(&self, _proc_id: Uuid, _component_id: Uuid) {}

  /// Sends a message through the IPC layer to the server side LongRunner. This creates a message ID
  /// used for routing any completed replies back to the sender's reducer.
  pub fn send(&self, sender: &Uuid, msg: JsValue) -> () {
    let msg_id = Uuid::new_v4();
    // Get the reducer for the sender.
    let reducers = self.reducers.read().unwrap();

    let _reducer = match reducers.get(sender) {
      Some(_) => {
        info!("Retrieved the reducer for {}", sender)
      }
      None => panic!("{} is not registered with the LongRunner", sender),
    };

    info!(
      "Sending a new message for {} with id {}:\n{:?}",
      sender, msg_id, msg
    );

    // Add a listener for the result and map them to the reducer
    let mut forwards = self.forwards.write().unwrap();
    let _ = forwards.insert(msg_id, vec![sender.clone()]);
    drop(forwards);

    // Send the mesage to the server side
    spawn_local(async move {
      info!("About to 'call_server::Heartbeat': {:#?}", msg);
      let reply = crate::invoke("call_server", msg).await;

      info!("Server sent the reply {:#?}", reply);
    });
  }

  pub fn receive(&self) {}
}
