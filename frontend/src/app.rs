use gloo_timers::callback::Timeout;
use js_sys::Function;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use tracing::info;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use jiradoro_common::prelude::*;

use crate::{
  components::{profile::*, timer_controls::*, timer_display::TimerDisplay},
  helpers::*,
};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TimerState {
  Paused,
  Running,
  Break,
}

#[derive(Serialize, Deserialize)]
struct SetTitleArgs<'a> {
  title: &'a str,
}

pub fn get_tray_title(timer_state: TimerState, timer_duration: u32, session_length: u32) -> String {
  match timer_state {
    TimerState::Paused => String::from("Paused"),
    TimerState::Running => {
      if timer_duration >= session_length {
        return format!("Finished session: {}", format_time(timer_duration));
      }
      return format!(
        "In session: {}",
        format_time(session_length - timer_duration)
      );
    }
    TimerState::Break => {
      if timer_duration >= session_length {
        return format!("Finished break: {}", format_time(timer_duration));
      }
      return format!("Break: {}", format_time(session_length - timer_duration));
    }
  }
}

#[function_component(CustardListener)]
fn custard_listener() -> Html {
  let oncustard = Callback::from(move |msg: Response| {
    info!("OnCustard received a message: {:#?}", msg);
  });

  use_effect(move || {
    let on_custard = Closure::<dyn FnMut(JsValue)>::new(move |raw| {
      info!("Received on_custard message: {:#?}", raw);
      let msg: EmissionEvent = serde_wasm_bindgen::from_value(raw).unwrap();
      oncustard.emit(msg.payload.message);
    });

    let unlisten = crate::listen_("Custard", &on_custard);
    let listener = (unlisten, on_custard);

    || {
      let promise = listener.0.clone();
      spawn_local(async move {
        info!("Spawned local listener for Custard");
        let unlisten: Function = wasm_bindgen_futures::JsFuture::from(promise)
          .await
          .unwrap()
          .into();
        unlisten.call0(&JsValue::undefined()).unwrap();
      });
      drop(listener);
    }
  });

  html! {
    <div>
      {"The listener goes here"}
    </div>
  }
}

#[function_component(App)]
pub fn app() -> Html {
  let session_length = yew::prelude::use_state(|| 25 * 60); // Default 25 minutes
  let timer_duration = yew::prelude::use_state(|| 0);
  let timer_state = yew::prelude::use_state(|| TimerState::Paused);

  use_effect_with(
    (
      timer_duration.clone(),
      timer_state.clone(),
      session_length.clone(),
    ),
    move |props| {
      let (timer_duration, timer_state, _) = props.clone();

      let timeout = Timeout::new(1_000, move || {
        if *timer_state != TimerState::Paused {
          timer_duration.set(*timer_duration + 1);
        }
      });

      let (timer_duration, timer_state, session_length) = props.clone();

      // Spawn a thread so that it can await the async call
      spawn_local(async move {
        let title = get_tray_title(*timer_state, *timer_duration, *session_length);

        let args = to_value(&SetTitleArgs { title: &title[..] }).unwrap();
        crate::invoke("set_title", args).await;
      });

      move || {
        timeout.cancel();
      }
    },
  );

  html! {
    <div class={classes!("h-screen", "flex", "flex-col")}>
      <CustardListener />
      <div class={classes!("h-fit", "w-full")}>
        <Profile button_status={Status::NotReady} />
      </div>
      <div class={classes!("flex", "items-center", "justify-center", "flex-col", "h-full")}>
          <TimerDisplay
            timer_state={timer_state.clone()}
            timer_duration={timer_duration.clone()}
            session_length={session_length.clone()}
          />
          <TimerControls
            session_length={session_length.clone()}
            timer_state={timer_state.clone()}
            timer_duration={timer_duration.clone()}
          />
      </div>
    </div>
  }
}
