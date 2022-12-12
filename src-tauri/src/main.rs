#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

extern crate core;

use tauri::{Manager, PhysicalSize, Size};

use crate::location::{get_filtered_locations, Location};
use crate::weather::{retrieve_weather, Weather};

mod weather;
mod location;

#[tauri::command]
async fn get_locations(query: String) -> Vec<Location> {
    return get_filtered_locations(query).await;
}

#[tauri::command]
async fn get_weather(lat: f32, lon: f32) -> Vec<Weather> {
    return retrieve_weather(lat, lon).await;
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.set_always_on_top(true).unwrap();
            let screen_size = Size::Physical(PhysicalSize { width: 350, height: 620 });
            main_window.set_max_size(Some(screen_size)).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_weather, get_locations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
