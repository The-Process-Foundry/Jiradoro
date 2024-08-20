// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
  menu::{MenuBuilder, MenuItemBuilder},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
  Emitter, Manager,
};
use tracing::info;
use uuid::Uuid;

use jiradoro_common::prelude::*;

mod longrunner;
pub use longrunner::prelude::*;

struct Server {
  pub counter: i32,
}

impl Server {
  fn handle(&self, msg: String) -> String {
    info!("Received msg: {}", msg);

    // rs2js(format!("{Count: {}", self.counter));
    format!("Counter Value {}", self.counter)
  }
}

struct State {
  server: Server,
}

#[tauri::command]
async fn set_title(_app_handle: tauri::AppHandle, _title: String) {
  #[cfg(target_os = "macos")]
  if let Err(e) = _app_handle.tray_handle().set_title(&_title) {
    eprintln!("error updating timer: {}", e);
  }
}

/// Receive a message from the client and forwards it along to the server side. Messages are passed
/// along serialized, leaving it to the server to fully process them.
#[tauri::command]
fn call_server(message: String, state: tauri::State<'_, State>, app: tauri::AppHandle) -> String {
  info!(?message, "Received tauri::command::call_server - ");
  // Let's emit an event that should be caught by the frontend window
  let guid = Uuid::new_v4();
  app
    .emit(
      "Emission",
      Emission {
        guid,
        message: Response::Ack(guid),
      },
    )
    .unwrap();

  state.server.handle(message)
}

fn main() {
  tracing_subscriber::fmt::init();

  let state = State {
    server: Server { counter: 0 },
  };

  tauri::Builder::default()
    .manage(state)
    .setup(|app| {
      let toggle = MenuItemBuilder::with_id("toggle", "Toggle").build(app)?;
      let menu = MenuBuilder::new(app).items(&[&toggle]).build()?;
      let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id().as_ref() {
          "toggle" => {
            println!("toggle clicked");
          }
          _ => (),
        })
        .on_tray_icon_event(|tray, event| {
          if let TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
          } = event
          {
            let app = tray.app_handle();
            if let Some(webview_window) = app.get_webview_window("main") {
              let _ = webview_window.show();
              let _ = webview_window.set_focus();
            }
          }
        })
        .build(app)?;

      // Automatically open the chrome dev-tools when building locally
      #[cfg(debug_assertions)]
      {
        let window = app.get_webview_window("main").unwrap();
        window.open_devtools();
        window.close_devtools();
      }

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![call_server, set_title])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
