use std::fs;
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

pub(crate) async fn get_filtered_locations(query: String) -> Vec<Location> {
    if query.is_empty() {
        return vec![];
    }
    let file = fs::read_to_string("database/cities.json").unwrap();
    let cities = serde_json::from_str::<Vec<Location>>(&*file).expect("JSON was not well-formatted");
    let filtered_cities: Vec<Location> = cities
        .into_iter()
        .filter(|e| e.name.to_lowercase().contains(&query.to_lowercase()))
        .collect();
    return filtered_cities;
}
