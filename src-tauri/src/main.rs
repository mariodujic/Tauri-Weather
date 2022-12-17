#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

extern crate core;

use tauri::{AppHandle, CustomMenuItem, Manager, PhysicalSize, Size, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, WindowBuilder, WindowUrl, Wry};

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

const OPEN_BUTTON_ID: &str = "open";
const QUIT_BUTTON_ID: &str = "quit";
const HIDE_BUTTON_ID: &str = "hide";
const MAIN_WINDOW_LABEL: &str = "main";
const APP_TITLE: &str = "Weather";
const MAX_SCREEN_SIZE: Size = Size::Physical(PhysicalSize { width: 350, height: 620 });

#[tokio::main]
async fn main() {
    let open = CustomMenuItem::new(OPEN_BUTTON_ID, "Open");
    let quit = CustomMenuItem::new(QUIT_BUTTON_ID, "Quit");
    let hide = CustomMenuItem::new(HIDE_BUTTON_ID, "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(open)
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    OPEN_BUTTON_ID => {
                        let window = app.get_window(MAIN_WINDOW_LABEL);
                        match window {
                            None => {
                                let win = WindowBuilder::new(
                                    app,
                                    MAIN_WINDOW_LABEL,
                                    WindowUrl::App("index.html".into()),
                                )
                                    .build()
                                    .unwrap();
                                win.set_max_size(Some(MAX_SCREEN_SIZE)).unwrap();
                                win.set_title(APP_TITLE).unwrap();
                                win.show().unwrap()
                            }
                            Some(win) => { win.show().unwrap(); }
                        }
                    }
                    QUIT_BUTTON_ID => {
                        std::process::exit(0);
                    }
                    HIDE_BUTTON_ID => {
                        let window = app.get_window(MAIN_WINDOW_LABEL);
                        match window {
                            Some(_) => { window.unwrap().hide().unwrap() }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        })
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
