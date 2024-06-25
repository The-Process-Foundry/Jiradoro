use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use tracing::info;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
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
pub fn Profile(_props: &Props) -> Html {
  let start_login: Callback<()> = {
    Callback::from(move |_| {
      info!("Received a click from the profile");
      spawn_local(async move {
        info!("Spawned Local");
        let args = to_value(&crate::Request {
          message: "Profile Click".to_string(),
        })
        .unwrap();
        info!("About to 'call_server' with args: {:#?}", args);
        crate::invoke("call_server", args).await;
      })
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
