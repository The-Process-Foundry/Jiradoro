use crate::{app::TimerState, helpers::format_time};
use yew::prelude::*;

// Define the component's properties
#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub timer_state: UseStateHandle<TimerState>,
  pub timer_duration: UseStateHandle<u32>,
  pub session_length: UseStateHandle<u32>,
}

#[function_component]
pub fn TimerDisplay(props: &Props) -> Html {
  let is_expired = *props.timer_duration > *props.session_length;

  let increase_session_length = {
    let session_length = props.session_length.clone();

    Callback::from(move |_: ()| {
      session_length.set(*session_length + 60 * 5); // Increase by 5 minutes
    })
  };

  let decrease_session_length = {
    let session_length = props.session_length.clone();
    Callback::from(move |_: ()| {
      session_length.set(*session_length - 60 * 5); // Decrease by 5 minutes
    })
  };

  let session_state_display = {
    let is_expired = *props.timer_duration > *props.session_length;

    let timer_state = props.timer_state.clone();
    match *timer_state {
      TimerState::Paused => "Paused".to_string(),
      TimerState::Break => {
        if is_expired {
          "Finished break".to_string()
        } else {
          "On break".to_string()
        }
      }
      TimerState::Running => {
        if is_expired {
          "Finished session".to_string()
        } else {
          "In session".to_string()
        }
      }
    }
  };

  let get_session_display: String = {
    let Props {
      timer_duration,
      session_length,
      ..
    } = props.clone();
    if is_expired {
      format_time(*timer_duration)
    } else {
      format_time(*session_length - *timer_duration)
    }
  };

  html! {
    <div class={classes!("flex", "flex-col", "space-y-2", "items-center")}>
        <div class={classes!("flex", "flex-row", "space-x-3")}>
            <button onclick={move |_| {
                decrease_session_length.emit(());
            }} class={classes!("p-2", "border-2", "border-red-500")}>
                { "- 5" }
            </button>
            <p class={classes!("text-5xl")}>
                {get_session_display}
            </p>
            <button onclick={move |_| {
                increase_session_length.emit(());
            }} class={classes!("p-2", "border-2", "border-green-500")}>
                { "+ 5" }
            </button>
        </div>
        {session_state_display}
    </div>
  }
}
