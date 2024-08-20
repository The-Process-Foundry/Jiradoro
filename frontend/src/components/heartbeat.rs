use gloo_timers::callback::{Interval, Timeout};
use serde_wasm_bindgen::to_value;
use std::{
  rc::Rc,
  sync::{Arc, Mutex},
};
use tracing::info;
use uuid::Uuid;
use yew::prelude::*;

use crate::prelude::*;
use jiradoro_common::prelude::*;

pub enum Msg {
  Ack(Uuid),
  IncrementTimer,
  HeartbeatClick,
  EndHeartbeat,
  /// A message received from the longrunner process. This is usually going to be a beat of the heart.
  Emission(String),
}

#[derive(Debug, PartialEq)]
enum HeartbeatStatus {
  Off,
  Started,
  Running(Uuid),
  Cancelling,
}

impl Default for HeartbeatStatus {
  fn default() -> HeartbeatStatus {
    HeartbeatStatus::Off
  }
}

#[derive(Debug, PartialEq)]
struct HeartbeatState {
  guid: Uuid,
  long_runner: LongRunnerCtx,
  status: HeartbeatStatus,
  text: &'static str,
}

impl Reducible for HeartbeatState {
  // Since this is stateful, this should always go to the "next" state without a need for an
  // explicit action
  type Action = Msg;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    match action {
      Msg::HeartbeatClick => match self.status {
        // Call to the RPC layer to start
        HeartbeatStatus::Off => {
          let msg = to_value(&Request {
            message: RequestMessage::Heartbeat,
          })
          .unwrap();
          self.long_runner.send(&self.guid, msg);

          HeartbeatState {
            long_runner: self.long_runner.clone(),
            guid: self.guid.clone(),
            status: HeartbeatStatus::Started,
            text: "Starting...",
          }
        }
        HeartbeatStatus::Started => HeartbeatState {
          long_runner: self.long_runner.clone(),
          guid: self.guid.clone(),
          status: HeartbeatStatus::Running(Uuid::new_v4()),
          text: "Cancel",
        },
        HeartbeatStatus::Running(_) => HeartbeatState {
          long_runner: self.long_runner.clone(),
          guid: self.guid.clone(),
          status: HeartbeatStatus::Cancelling,
          text: "Stopping ...",
        },
        HeartbeatStatus::Cancelling => HeartbeatState {
          long_runner: self.long_runner.clone(),
          guid: self.guid.clone(),
          status: HeartbeatStatus::Off,
          text: "Start",
        },
      }
      .into(),
      Msg::Emission(msg) => {
        info!("Received an emission: {}", msg);
        self.into()
      }
      _ => {
        info!("Got an unhandled type of message");
        self.into()
      }
    }
  }
}

#[derive(Debug, Default)]
pub struct HeartbeatData {
  pub guid: Uuid,
  pub time_elapsed: f32,
  pub count: i32,
  pub server_guid: Option<Uuid>,
  pub interval: Option<Interval>,
  pub timeout: Option<Timeout>,
}

#[function_component]
pub fn Heartbeat() -> Html {
  // Start the timeout to fire a warning after the elapsed time.
  // Start the interval for updating the page with the latest timeout.

  let data = HeartbeatData::default();

  // Makes a reference identifier for this component.
  let guid = Uuid::new_v4();

  // Get the long runner used to send messages
  let long_runner = use_context::<LongRunnerCtx>().expect("LongRunner context not found");

  // Setup the initial state and the dispatcher for this component
  let heartbeat_state: UseReducerHandle<HeartbeatState> = use_reducer_eq(move || HeartbeatState {
    long_runner,
    guid,
    status: HeartbeatStatus::default(),
    text: "Start",
  });

  // A callback to feed to the longrunner. This is kept simple, as the reducer itself should
  // shoulder most of the burden for updating the component here.
  let on_heartbeat: Callback<String> = {
    let state = heartbeat_state.clone();

    Callback::from(move |msg| {
      info!("LongRunner is sending back a message to the reducer");
      state.dispatch(Msg::Emission(msg))
    })
  };

  // Register the reducer for this component with the long_runner
  let _ = heartbeat_state.long_runner.register(guid, on_heartbeat);

  let on_click: Callback<()> = {
    let state = heartbeat_state.clone();
    info!("Clicked button: {:?}", state.status);

    Callback::from(move |_| state.dispatch(Msg::HeartbeatClick))
  };

  html! (
      <div>
          <div>
              {"Heart has a beat "}<b>{data.count}</b>{" times."}
          </div>
          <div>
              {"It has been "}<b>{format!("{:.2}", data.time_elapsed)}</b>{" seconds since the last heartbeat."}
          </div>
          <div>
              <button
                  class={classes!("cursor-pointer", "border-2", "text-gray", "p-2")}
                  onclick={move |_| {on_click.emit(())}}
              >{heartbeat_state.text}</button>
          </div>
      </div>
  )
}
