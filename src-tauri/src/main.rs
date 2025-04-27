#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{thread, time::Duration};
use tauri::{Builder, Emitter};

fn main() {
    Builder::default()
        .setup(|app| {
          let app_handle = app.handle().clone();

            thread::spawn(move || {
                let device_state = DeviceState::new();
                let mut prev_keys: Vec<Keycode> = vec![];

                loop {
                    let keys = device_state.get_keys();

                    for key in &keys {
                        if !prev_keys.contains(key) {
                            match key {
                                //letras
                                Keycode::A | Keycode::B | Keycode::C | Keycode::D | Keycode::E |
                                Keycode::F | Keycode::G | Keycode::H | Keycode::I | Keycode::J |
                                Keycode::K | Keycode::L | Keycode::M | Keycode::N | Keycode::O |
                                Keycode::P | Keycode::Q | Keycode::R | Keycode::S | Keycode::T |
                                Keycode::U | Keycode::V | Keycode::W | Keycode::X | Keycode::Y |
                                Keycode::Z => {
                                    let letter = format!("{:?}", key).to_lowercase();
                                    let _ = app_handle.emit_to("main", "key-pressed", letter);
                                }

                                //Números
                                Keycode::Key0 | Keycode::Key1 |  Keycode::Key2 |  Keycode::Key3 |
                                Keycode::Key4 | Keycode::Key5 |  Keycode::Key6 |  Keycode::Key7 |
                                Keycode::Key8 | Keycode::Key9 => {
                                  let number = format!("{:?}", key).replace("Key", "");
                                  let _ = app_handle.emit_to("main", "key-pressed", number);
                                }

                                //Espacio
                                Keycode::Space => {
                                  let _ = app_handle.emit_to("main", "key-pressed", " ".to_string());
                                }

                                //Backspace
                                Keycode::Backspace => {
                                  let _ = app_handle.emit_to("main", "key-pressed", "backspace".to_string());
                                }

                                //Todos los demás símbolos
                                other => {
                                  let symbol = format!("{:?}", other).to_lowercase();
                                  let _ = app_handle.emit_to("main", "key-pressed", symbol);
                                }
                            }
                        }
                    }

                    prev_keys = keys;
                    thread::sleep(Duration::from_millis(50));
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
