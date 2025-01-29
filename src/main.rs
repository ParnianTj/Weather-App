use dotenv::dotenv;
use notify_rust::Notification;
use reqwest::Error;
use serde::Deserialize;
use std::env;
use std::time::Duration;
use tokio;

#[derive(Deserialize, Debug, Clone)]
struct WeatherData {
    WeatherText: String,
    Temperature: Temperature,
    UVIndex: u8,
    AirQuality: Option<AirQuality>,
}

#[derive(Deserialize, Debug, Clone)]
struct Temperature {
    Metric: Metric,
}

#[derive(Deserialize, Debug, Clone)]
struct Metric {
    Value: f32,
}

#[derive(Deserialize, Debug, Clone)]
struct AirQuality {
    Category: String,
}

fn get_api_key() -> String {
    dotenv().ok();
    env::var("ACCUWEATHER_API_KEY").expect("ACCUWEATHER_API_KEY not set in .env")
}

async fn fetch_weather(api_key: &str, location_key: &str) -> Result<WeatherData, Error> {
    let url = format!(
        "http://dataservice.accuweather.com/currentconditions/v1/{}?apikey={}&details=true",
        location_key, api_key
    );
    let response = reqwest::get(&url).await?.json::<Vec<WeatherData>>().await?;
    Ok(response[0].clone())
}

fn is_severe_weather(weather: &WeatherData) -> bool {
    let severe_conditions = ["Storm", "Hail", "Tornado"];
    severe_conditions.contains(&weather.WeatherText.as_str())
}

fn is_high_uv(weather: &WeatherData) -> bool {
    weather.UVIndex >= 8
}

fn is_polluted_air(weather: &WeatherData) -> bool {
    if let Some(air_quality) = &weather.AirQuality {
        air_quality.Category == "Unhealthy"
    } else {
        false
    }
}

fn send_notification(weather: &WeatherData) {
    let air_quality_message = match &weather.AirQuality {
        Some(air_quality) => format!("Air Quality: {}", air_quality.Category),
        None => "Air Quality: Not available".to_string(),
    };

    let notification_message = format!(
        "Weather: {}\nTemperature: {}°C\nUV Index: {}\n{}",
        weather.WeatherText, weather.Temperature.Metric.Value, weather.UVIndex, air_quality_message
    );

    Notification::new()
        .summary("Weather Alert")
        .body(&notification_message)
        .show()
        .unwrap();
}

async fn run_notifier() {
    let api_key = get_api_key();
    let location_key = "178086"; // Replaced with actual key for Munich, Germany
    
    println!("Starting weather notifier...");
    
    loop {
        println!("Fetching weather data...");
        match fetch_weather(&api_key, location_key).await {
            Ok(weather) => {
                println!("Weather data fetched successfully: {:?}", weather);
    
                if is_severe_weather(&weather) {
                    println!("Severe weather detected!");
                    send_notification(&weather);
                }
                if is_high_uv(&weather) {
                    println!("High UV detected!");
                    send_notification(&weather);
                }
                if is_polluted_air(&weather) {
                    println!("Polluted air detected!");
                    send_notification(&weather);
                }
                // Always send the detailed weather update
                send_notification(&weather);
            }
            Err(e) => {
                eprintln!("Failed to fetch weather data: {}", e);
            }
        }
        println!("Waiting for the next check...");
        tokio::time::sleep(Duration::from_secs(300)).await; // Checks every 5 minutes
    }
}

#[tokio::main]
async fn main() {
    run_notifier().await;
}