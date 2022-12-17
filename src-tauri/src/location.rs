use std::fs;
use std::path::PathBuf;
use std::string::String;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub country: String,
    pub admin1: Option<String>,
    pub lat: String,
    pub lon: String,
}

pub(crate) async fn get_filtered_locations(query: String, resource_path: PathBuf) -> Vec<Location> {
    if query.is_empty() {
        return vec![];
    }
    let file_content = fs::read_to_string(resource_path.to_str().unwrap()).unwrap();
    let cities = serde_json::from_str::<Vec<Location>>(&*file_content).expect("JSON was not well-formatted");
    let filtered_cities: Vec<Location> = cities
        .into_iter()
        .filter(|e| e.name.to_lowercase().contains(&query.to_lowercase()))
        .collect();
    return filtered_cities;
}
