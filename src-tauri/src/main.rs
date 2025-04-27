#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{thread, time::Duration};
use tauri::{
  menu::{Menu, MenuItem}, tray::TrayIconBuilder, Builder, Emitter, Manager
};

fn main() {
    
    Builder::default()
        .setup(|app| {
          let app_handle = app.handle();

               // Crear los items de menú
               let show_item = MenuItem::with_id(app_handle, "show", "Show", true, None::<&str>)?;
               let hide_item = MenuItem::with_id(app_handle, "hide", "Hide", true, None::<&str>)?;
               let quit_item = MenuItem::with_id(app_handle, "quit", "Quit", true, None::<&str>)?;
   
               // Crear el menú
               let menu = Menu::with_items(app_handle, &[&show_item, &hide_item, &quit_item])?;
   
               // Crear el tray icon
               TrayIconBuilder::new()
                   .menu(&menu)
                   .on_menu_event(|app, event| match event.id.as_ref() {
                       "show" => {
                           println!("Show clicked");
                           if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                           }
                           
                       }
                       "hide" => {
                           println!("Hide clicked");
                           if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                           }
                       }
                       "quit" => {
                           println!("Quit clicked");
                           app.exit(0);
                       }
                       _ => {
                           println!("Menu item {:?} not handled", event.id);
                       }
                   })
                   .icon(app_handle.default_window_icon().unwrap().clone())
                   .build(app_handle)
                   .expect("failed to build tray icon");

          let app_handle_clone = app.handle().clone();

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
                                    let _ = app_handle_clone.emit_to("main", "key-pressed", letter);
                                }

                                //Números
                                Keycode::Key0 | Keycode::Key1 |  Keycode::Key2 |  Keycode::Key3 |
                                Keycode::Key4 | Keycode::Key5 |  Keycode::Key6 |  Keycode::Key7 |
                                Keycode::Key8 | Keycode::Key9 => {
                                  let number = format!("{:?}", key).replace("Key", "");
                                  let _ = app_handle_clone.emit_to("main", "key-pressed", number);
                                }

                                //Espacio
                                Keycode::Space => {
                                  let _ = app_handle_clone.emit_to("main", "key-pressed", " ".to_string());
                                }

                                //Backspace
                                Keycode::Backspace => {
                                  let _ = app_handle_clone.emit_to("main", "key-pressed", "backspace".to_string());
                                }

                                //enter
                                Keycode::Enter => {
                                  let _ = app_handle_clone.emit_to("main", "key-pressed", "space".to_string());
                                }

                                //Interrogación
                                Keycode::Slash | Keycode::Minus => {
                                  let pressed_keys = device_state.get_keys();
                                  if pressed_keys.contains(&Keycode::LShift) || pressed_keys.contains(&Keycode::RShift) {
                                    let _ = app_handle_clone.emit_to("main", "key-pressed", "?".to_string());
                                  } else {
                                    let _ = app_handle_clone.emit_to("main", "key-pressed", "symbol".to_string());
                                  }
                                }

                                //Todos los demás símbolos
                                other => {
                                  let symbol = format!("{:?}", other).to_lowercase();
                                  let _ = app_handle_clone.emit_to("main", "key-pressed", symbol);
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
        .on_window_event(|app_handle, event| {
          if let tauri::WindowEvent::CloseRequested { api, .. } = event{
            // Prevent the window from closing
            // and hide it instead.
            if let Some(window) = app_handle.get_webview_window("main") {
                let _ = window.hide();
            }
            api.prevent_close();
          }
        })
      
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
