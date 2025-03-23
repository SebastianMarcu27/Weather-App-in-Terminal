use chrono::{DateTime, Local};
use serde::Deserialize;
use std::error::Error;

#[derive(Debug)]
pub struct CityInfo {
    pub city: String,
    pub temperature: f64,
    pub wind_speed: f64,
    pub humidity: u8,
    pub time: DateTime<Local>,
}

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    name: String,
    main: MainInfo,
    wind: WindInfo,
}

#[derive(Deserialize, Debug)]
struct MainInfo {
    temp: f64,
    humidity: u8,
}

#[derive(Deserialize, Debug)]
struct WindInfo {
    speed: f64,
}

pub async fn get_data(city: String) -> Result<CityInfo, Box<dyn Error>> {
    let api_key = std::env::var("OPENWEATHER_API_KEY")
        .unwrap_or_else(|_| "9f8ed14f672b974e0d0d84fa7523ff5e".to_string());
    

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}&units=metric"
    );

    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let data: WeatherResponse = response.json().await?;
        Ok(CityInfo {
            city: data.name,
            temperature: data.main.temp,
            wind_speed: data.wind.speed,
            humidity: data.main.humidity,
            time: Local::now(),
        })
    } else {
        Err(format!("Failed to fetch data: {}", response.status()).into())
    }
}
