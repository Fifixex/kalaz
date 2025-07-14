use std::{fs::File, io::BufReader, sync::Mutex, thread};

use once_cell::sync::Lazy;
use rdev::{listen, EventType};
use rodio::{Decoder, OutputStream, Sink};
use tauri::{path::BaseDirectory, AppHandle, Emitter, Manager, Runtime};

static CURRENT_SOUND: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new("key.mp3".to_string())
});

#[tauri::command]
fn set_active_sound<R: Runtime>(app_handle: AppHandle<R>, sound: String) {
    println!("sonido seleccionado: {}", sound);
    play_sound(&app_handle, &sound);
    let mut current = CURRENT_SOUND.lock().unwrap();
    *current = sound;
}

fn play_sound<R: Runtime>(app_handle: &AppHandle<R>, file_name: &str) {
    let path = app_handle
        .path()
        .resolve(&format!("sounds/{}", file_name), BaseDirectory::Resource)
        .expect("No he encontrado la ruta");

    println!("Intentando reproducir: {:?}", path);

    match OutputStream::try_default() {
        Ok((_stream, handle)) => {
            println!("Stream de audio creado");
            match File::open(&path) {
                Ok(file) => {
                    let source = match Decoder::new(BufReader::new(file)) {
                        Ok(src) => src,
                        Err(e) => {
                            eprintln!("Error al decodificar: {:?}", e);
                            return;
                        }
                    };
                    let sink = Sink::try_new(&handle).unwrap();
                    sink.append(source);
                    sink.sleep_until_end();
                }
                Err(e) => eprintln!("Error abriendo archivo de sonido: {:?}", e),
            }
        }
        Err(e) => eprintln!("Error creando stream de audio: {:?}", e),
    }
}

fn start_keyboard_listener<R: Runtime>(app_handle: AppHandle<R>) {
    thread::spawn(move || {
        println!("iniciando keyboard listener");

        let result = listen(move |event| {
            if let EventType::KeyPress(key) = event.event_type {
                let sound = CURRENT_SOUND.lock().unwrap().clone();
                play_sound(&app_handle, &sound);
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
