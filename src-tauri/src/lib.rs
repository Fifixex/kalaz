use std::thread;

use rdev::{listen, EventType};
use tauri::{AppHandle, Emitter};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn start_keyboard_listener(app_handle: AppHandle) {
  thread::spawn(move || {
    let result = listen(move |event| {
      if let EventType::KeyPress(key) = event.event_type {
        app_handle.emit("key_pressed", format!("{:?}", key)).unwrap();
      }
    });

    if let Err(err) = result {
      eprintln!("{:?}", err);
    }
  });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
          let app_handle = app.handle();
          start_keyboard_listener(app_handle.clone());
          Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
