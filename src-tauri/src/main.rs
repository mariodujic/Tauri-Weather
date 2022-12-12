#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

extern crate core;

use std::fs;
use std::string::String;

use chrono::Timelike;
use dateparser::parse;
use serde::{Deserialize, Serialize};
use tauri::{Manager, PhysicalSize, Size};

use crate::location::Location;

mod weather;
mod location;

#[tauri::command]
async fn get_locations(location: String) -> Vec<Location> {
    let file = fs::read_to_string("database/cities.json").unwrap();
    let cities = serde_json::from_str::<Vec<Location>>(&*file).expect("JSON was not well-formatted");
    let filtered_cities: Vec<Location> = cities
        .into_iter()
        .filter(|e| e.name.to_lowercase().contains(&location.to_lowercase()))
        .collect();
    return filtered_cities;
}

#[derive(Serialize, Deserialize, Debug)]
struct Day {
    temperature: f64,
    time: String,
    icon: String,
}

#[tauri::command]
async fn get_weather(lat: f32, lon: f32) -> Vec<Day> {
    let weather = weather::get_weather(lat, lon).await;
    return if weather.is_ok() {
        let _weather = weather.unwrap();

        let temperature: Vec<Day> = _weather.properties.timeseries
            .into_iter()
            .enumerate()
            .filter(|(i, e)| {
                let temperature_time = &e.time;
                let temperature_time = parse(&*temperature_time).unwrap();
                *i == 0 || temperature_time.hour() == 12
            })
            .map(|(_, e)| {
                let next_hour = e.data.next_1_hours;
                let next_six_hours = e.data.next_6_hours;
                let next_twelve_hours = e.data.next_12_hours;

                let icon = if next_hour.is_some() {
                    next_hour.unwrap().summary.symbol_code
                } else if next_six_hours.is_some() {
                    next_six_hours.unwrap().summary.symbol_code
                } else if next_twelve_hours.is_some() {
                    next_twelve_hours.unwrap().summary.symbol_code
                } else {
                    String::from("")
                };
                let temperature = e.data.instant.details.air_temperature;
                let temperature_time = e.time;
                Day { temperature, time: temperature_time, icon }
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
            let screen_size = Size::Physical(PhysicalSize { width: 350, height: 620 });
            main_window.set_max_size(Some(screen_size)).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_weather, get_locations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
