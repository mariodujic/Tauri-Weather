#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::string::String;

use serde::{Deserialize, Serialize};
use tauri::{Manager, PhysicalSize, Size};

mod weather;

#[derive(Serialize, Deserialize, Debug)]
struct Day {
    temperature: f64,
    time: String,
    icon: String,
}

#[tauri::command]
async fn get_weather() -> Vec<Day> {
    let weather = weather::get_weather().await;
    return if weather.is_ok() {
        let _weather = weather.unwrap();
        let temperature: Vec<Day> = _weather.properties.timeseries
            .into_iter()
            .map(|e| {
                let temperature = e.data.instant.details.air_temperature;
                let time = e.time;
                let next_hour = e.data.next_1_hours;
                let icon = if next_hour.is_some() {
                    next_hour.unwrap().summary.symbol_code
                } else {
                    String::new()
                };
                Day { temperature, time, icon }
            })
            .collect();
        temperature
    } else {
        vec![]
    };
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.set_always_on_top(true).unwrap();
            main_window.set_max_size(Option::Some(Size::Physical(PhysicalSize { width: 350, height: 620 }))).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_weather])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
