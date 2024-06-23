mod app;
pub mod components;

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

fn main() {
  // Enable Console.log for displaying tracing messages before anything else
  logger::init();
  tracing::info!("Starting the app");
  yew::Renderer::<app::App>::new().render();
}
