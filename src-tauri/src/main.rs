// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use load_dotenv::load_dotenv;
use log::*;
use reqwest::Response;
use tauri::{App, Manager, Window};

load_dotenv!();

#[tauri::command]
fn test_event(window: Window) {
    window.emit_and_trigger("event-name", "Event Gay").unwrap();
    info!("Event emitted");
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            test_event
        ])
        .setup(|app| {
            info!("Starting tauri app");
            dotenv::dotenv().ok();
            let main_window = app.get_window("main").unwrap();
            let main_window_clone = main_window.clone();
            std::thread::spawn(move || {
                let id = main_window.listen("event-name", |event| {
                    info!("got event-name with payload {:?}", event.payload());
                });
            });

            // main_window_clone
            //     .emit("event-name", "event from backend")
            //     .unwrap();
            // tauri::async_runtime::spawn(jojo_server::initialize());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
