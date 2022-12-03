use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub geometry: Geometry,
    pub properties: Properties,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geometry {
    pub coordinates: Vec<f64>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    pub meta: Meta,
    pub timeseries: Vec<TimeSerie>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub units: Units,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Units {
    #[serde(rename = "air_pressure_at_sea_level")]
    pub air_pressure_at_sea_level: String,
    #[serde(rename = "air_temperature")]
    pub air_temperature: String,
    #[serde(rename = "cloud_area_fraction")]
    pub cloud_area_fraction: String,
    #[serde(rename = "precipitation_amount")]
    pub precipitation_amount: String,
    #[serde(rename = "relative_humidity")]
    pub relative_humidity: String,
    #[serde(rename = "wind_from_direction")]
    pub wind_from_direction: String,
    #[serde(rename = "wind_speed")]
    pub wind_speed: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSerie {
    pub time: String,
    pub data: Data,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub instant: Instant,
    #[serde(rename = "next_12_hours")]
    pub next_12_hours: Option<Next12Hours>,
    #[serde(rename = "next_1_hours")]
    pub next_1_hours: Option<Next1Hours>,
    #[serde(rename = "next_6_hours")]
    pub next_6_hours: Option<Next6Hours>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instant {
    pub details: Details,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    #[serde(rename = "air_pressure_at_sea_level")]
    pub air_pressure_at_sea_level: f64,
    #[serde(rename = "air_temperature")]
    pub air_temperature: f64,
    #[serde(rename = "cloud_area_fraction")]
    pub cloud_area_fraction: f64,
    #[serde(rename = "relative_humidity")]
    pub relative_humidity: f64,
    #[serde(rename = "wind_from_direction")]
    pub wind_from_direction: f64,
    #[serde(rename = "wind_speed")]
    pub wind_speed: f64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Next12Hours {
    pub summary: Summary,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    #[serde(rename = "symbol_code")]
    pub symbol_code: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Next1Hours {
    pub summary: Summary2,
    pub details: Details2,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary2 {
    #[serde(rename = "symbol_code")]
    pub symbol_code: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details2 {
    #[serde(rename = "precipitation_amount")]
    pub precipitation_amount: f64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Next6Hours {
    pub summary: Summary3,
    pub details: Details3,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary3 {
    #[serde(rename = "symbol_code")]
    pub symbol_code: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details3 {
    #[serde(rename = "precipitation_amount")]
    pub precipitation_amount: f64,
}

const URL: &str = "https://api.met.no/weatherapi/locationforecast/2.0/compact";

pub(crate) async fn get_weather(lat: f32, lon: f32) -> Result<Weather, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(URL)
        .query(&[("lat", lat), ("lon", lon)])
        .header(USER_AGENT, "My Rust Program 1.0")
        .send()
        .await?
        .text()
        .await?;
    let weather: Weather = serde_json::from_str(&body)?;
    Ok(weather)
}