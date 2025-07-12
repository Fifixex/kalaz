use std::{fs::File, io::BufReader, path::Path, sync::Mutex, thread};

use once_cell::sync::Lazy;
use rdev::{listen, EventType};
use rodio::{Decoder, OutputStream, Sink};
use tauri::{Emitter, AppHandle};

static CURRENT_SOUND: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new("cherry-mx-blue.ogg".to_string())
});

#[tauri::command]
fn set_active_sound(sound: String) {
    println!("sonido: {}", sound);
    let mut current = CURRENT_SOUND.lock().unwrap();
    *current = sound;
}

fn play_sound(file_name: &str) {
    println!("play: {}", file_name);
    let path = Path::new("src-tauri/sounds").join(file_name);
    println!("Full path: {:?}", path);

    if let Ok((_stream, handle)) = OutputStream::try_default() {
        if let Ok(file) = File::open(path) {
            let sink = Sink::try_new(&handle).unwrap();
            let source = Decoder::new(BufReader::new(file)).unwrap();
            sink.append(source);
            sink.detach();
        }
    }
}

fn start_keyboard_listener(app_handle: AppHandle) {
    thread::spawn(move || {
        println!("iniciando keyboard listener");
        let result = listen(move |event| {
            if let EventType::KeyPress(key) = event.event_type {
                let sound = CURRENT_SOUND.lock().unwrap().clone();
                play_sound(&sound);
                app_handle.emit("key_pressed", format!("{:?}", key)).unwrap();
            }
        });

        if let Err(e) = result {
            eprintln!("Error capturando teclas: {:?}", e);
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
        .invoke_handler(tauri::generate_handler![set_active_sound])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
