use std::rc::Rc;
use tracing::info;
use uuid::Uuid;
use yew::prelude::*;

use gloo_timers::callback::{Interval, Timeout};

pub enum Msg {
  IncrementTimer,
  HeartbeatClick,
  EndHeartbeat,
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
  status: HeartbeatStatus,
  text: &'static str,
}

impl Default for HeartbeatState {
  fn default() -> HeartbeatState {
    HeartbeatState {
      status: HeartbeatStatus::default(),
      text: "Start",
    }
  }
}

impl Reducible for HeartbeatState {
  // Since this is stateful, this should always go to the "next" state without a need for an
  // explicit action
  type Action = Msg;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    match action {
      Msg::HeartbeatClick => match self.status {
        HeartbeatStatus::Off => HeartbeatState {
          status: HeartbeatStatus::Started,
          text: "Starting...",
        },
        HeartbeatStatus::Started => HeartbeatState {
          status: HeartbeatStatus::Running(Uuid::new_v4()),
          text: "Cancel",
        },
        HeartbeatStatus::Running(_) => HeartbeatState {
          status: HeartbeatStatus::Cancelling,
          text: "Stopping ...",
        },
        HeartbeatStatus::Cancelling => HeartbeatState {
          status: HeartbeatStatus::Off,
          text: "Start",
        },
      }
      .into(),
      _ => self.into(),
    }
  }
}

#[derive(Debug, Default)]
pub struct HeartbeatData {
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

  let heartbeat_state: UseReducerHandle<HeartbeatState> = use_reducer_eq(HeartbeatState::default);

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
