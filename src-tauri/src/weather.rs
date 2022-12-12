use chrono::Timelike;
use dateparser::parse;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherResponse {
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

pub(crate) async fn get_weather(lat: f32, lon: f32) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(URL)
        .query(&[("lat", lat), ("lon", lon)])
        .header(USER_AGENT, "My Rust Program 1.0")
        .send()
        .await?
        .text()
        .await?;
    let weather: WeatherResponse = serde_json::from_str(&body)?;
    Ok(weather)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    temperature: f64,
    time: String,
    icon: String,
}

pub(crate) async fn retrieve_weather(lat: f32, lon: f32) -> Vec<Weather> {
    let weather = get_weather(lat, lon).await;
    return if weather.is_ok() {
        let _weather = weather.unwrap();

        let temperature: Vec<Weather> = _weather.properties.timeseries
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
                Weather { temperature, time: temperature_time, icon }
            })
            .collect();
        temperature
    } else {
        vec![]
    };
}
