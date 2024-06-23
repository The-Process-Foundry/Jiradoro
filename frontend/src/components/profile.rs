use tracing::info;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Status {
  NotReady,
  Ready,
  Running,
  Finished,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
  pub button_status: Status,
}

#[function_component]
pub fn Profile(props: &Props) -> Html {
  let start_login: Callback<()> = {
    Callback::from(move |_| {
      info!("Received a click from the profile");
    })
  };

  html! (
      <div class={classes!("text-right", "p-4", "cursor-pointer")}>
        <button
            class={classes!("cursor-pointer", "border-2", "text-gray", "p-2")}
            onclick={move |_| {start_login.emit(())}}
        >
            {"profile button"}
        </button>
      </div>
  )
}
