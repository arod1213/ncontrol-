#![cfg_attr(
    all(not(debug_assertions), target_os = "macos"),
    windows_subsystem = "macos"
)]

use tauri::{SystemTray, SystemTrayMenu, SystemTrayEvent, CustomMenuItem, Manager};
use std::sync::{Arc, Mutex};

fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("quit", "Quit"));

    tauri::Builder::default()
        .setup(|_app| {
            // Your background thread to handle MIDI control
            std::thread::spawn(|| {
                // Your VolumeController logic here
            });
            Ok(())
        })
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                if id.as_str() == "quit" {
                    std::process::exit(0);
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}
