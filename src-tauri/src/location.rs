use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub country: String,
    pub admin1: Option<String>,
    pub lat: String,
    pub lon: String
}