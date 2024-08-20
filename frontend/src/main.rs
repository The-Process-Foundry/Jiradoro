use js_sys::Promise;
use wasm_bindgen::prelude::*;

mod app;
pub mod components;

pub mod longrunner;

pub mod prelude {
  pub use super::longrunner::LongRunnerCtx;
}

mod helpers {
  pub fn format_time(seconds: u32) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
  }
}

/// Enable tracing to dump to the console
mod logger {
  use tracing_subscriber::{
    filter::filter_fn,
    fmt::format::{FmtSpan, Pretty},
    prelude::*,
  };
  use tracing_web::{performance_layer, MakeConsoleWriter};

  pub(crate) fn init() {
    let fmt_layer = tracing_subscriber::fmt::layer()
      .with_ansi(false)
      .without_time()
      .with_writer(MakeConsoleWriter)
      .with_span_events(FmtSpan::ACTIVE)
      .with_filter(filter_fn(|metadata| {
        metadata.target().starts_with("jiradoro_gui")
      }));

    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
      .with(fmt_layer)
      .with(perf_layer)
      .init();
  }
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Request {
//   message: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Response {
//   message: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Emission {
//   payload: Response,
// }

// Defines an async Rust function to call Tauri, used for updating the system tray state.
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window.__TAURI__.core"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;
  #[wasm_bindgen(js_namespace = ["window.__TAURI__.event"], js_name = "listen")]
  fn listen_(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> Promise;
}

fn main() {
  // Enable Console.log for displaying tracing messages before anything else
  logger::init();
  tracing::info!("Starting the app");
  yew::Renderer::<app::App>::new().render();
}
