#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

extern crate core;

use tauri::{AppHandle, Manager, PhysicalSize, Size, Wry};

use crate::location::{get_filtered_locations, Location};
use crate::weather::{retrieve_weather, Weather};

mod weather;
mod location;

#[tauri::command]
async fn get_locations(query: String, handle: AppHandle<Wry>) -> Vec<Location> {

    let resource_path = handle.path_resolver()
        .resolve_resource("database/cities.json")
        .expect("Failed to resolve resource");

    return get_filtered_locations(query, resource_path).await;
}

#[tauri::command]
async fn get_weather(lat: f32, lon: f32) -> Vec<Weather> {
    return retrieve_weather(lat, lon).await;
}

const MAIN_WINDOW_LABEL: &str = "main";
const APP_TITLE: &str = "Weather";
const MAX_SCREEN_SIZE: Size = Size::Physical(PhysicalSize { width: 350, height: 620 });

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window(MAIN_WINDOW_LABEL).unwrap();
            main_window.set_title(APP_TITLE).unwrap();
            main_window.set_max_size(Some(MAX_SCREEN_SIZE)).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_weather, get_locations])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
