use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct City {
    pub name: String,
    pub country: String,
    #[serde(rename = "admin1")]
    pub administration: Option<String>,
    pub lat: String,
    pub lon: String
}