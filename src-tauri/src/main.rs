// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

use tracing::info;

struct Server {
  pub counter: i32,
}

impl Server {
  fn handle(&self, msg: String) -> String {
    info!("Received msg: {}", msg);
    // rs2js(format!("{Count: {}", self.counter));
    format!("{}", self.counter)
  }
}

struct State {
  server: Server,
}

#[tauri::command]
async fn set_title(app_handle: tauri::AppHandle, title: String) {
  #[cfg(target_os = "macos")]
  if let Err(e) = app_handle.tray_handle().set_title(&title) {
    eprintln!("error updating timer: {}", e);
  }
}

/// Receive a message from the client and forwards it along to the server side. Messages are passed
/// along serialized, leaving it to the server to fully process them.
#[tauri::command]
fn call_server(message: String, state: tauri::State<'_, State>) -> String {
  info!(?message, "Received tauri::command: call_server");

  state.server.handle(message)
}

fn main() {
  tracing_subscriber::fmt::init();

  let tray = SystemTray::new();
  let state = State {
    server: Server { counter: 0 },
  };

  tauri::Builder::default()
    .manage(state)
    .setup(|app| {
      // Automatically open the chrome dev-tools when building locally
      #[cfg(debug_assertions)]
      {
        let window = app.get_window("main").unwrap();
        window.open_devtools();
        window.close_devtools();
      }
      Ok(())
    })
    .system_tray(tray)
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a left click");
        let window = app.get_window("main").unwrap();
        window.show().unwrap();
      }
      _ => (),
    })
    .invoke_handler(tauri::generate_handler![call_server, set_title])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
