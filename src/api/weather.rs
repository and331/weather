use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

const GEOCODING_API: &str = "https://geocoding-api.open-meteo.com/v1/search";
const WEATHER_API: &str = "https://api.open-meteo.com/v1/forecast";

// Структура для пошуку міста
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeocodingResponse {
    pub results: Option<Vec<GeoLocation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub country: String,
    pub admin1: Option<String>,
}

// Структура погодних даних
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherResponse {
    pub current: CurrentWeather,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentWeather {
    pub temperature_2m: f64,
    pub relative_humidity_2m: i32,
    pub apparent_temperature: f64,
    pub weather_code: i32,
    pub wind_speed_10m: f64,
    pub surface_pressure: f64,
}

// Об'єднана структура для компонента
#[derive(Debug, Clone)]
pub struct WeatherData {
    pub name: String,
    pub country: String,
    pub main: MainWeather,
    pub weather: WeatherInfo,
    pub wind: Wind,
    pub sys: Sys,
    pub visibility: i32,
}

#[derive(Debug, Clone)]
pub struct MainWeather {
    pub temp: f64,
    pub feels_like: f64,
    pub humidity: i32,
    pub pressure: i32,
}

#[derive(Debug, Clone)]
pub struct WeatherInfo {
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Clone)]
pub struct Wind {
    pub speed: f64,
}

#[derive(Debug, Clone)]
pub struct Sys {
    pub country: String,
}

// Функція для отримання опису погоди за кодом WMO
fn get_weather_description(code: i32) -> (&'static str, &'static str) {
    match code {
        0 => ("Ясно", "01d"),
        1 | 2 => ("Переважно ясно", "02d"),
        3 => ("Хмарно", "03d"),
        45 | 48 => ("Туман", "50d"),
        51 | 53 | 55 => ("Мряка", "09d"),
        56 | 57 => ("Ожеледь", "13d"),
        61 | 63 | 65 => ("Дощ", "10d"),
        66 | 67 => ("Мокрий сніг", "13d"),
        71 | 73 | 75 => ("Сніг", "13d"),
        77 => ("Град", "13d"),
        80 | 81 | 82 => ("Зливи", "09d"),
        85 | 86 => ("Снігопад", "13d"),
        95 => ("Гроза", "11d"),
        96 | 99 => ("Гроза з градом", "11d"),
        _ => ("Невідомо", "01d"),
    }
}

async fn get_coordinates(city: &str) -> Result<GeoLocation, String> {
    let url = format!(
        "{}?name={}&count=1&language=uk&format=json",
        GEOCODING_API, 
        urlencoding::encode(city)
    );

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Помилка мережі: {}", e))?;

    if !response.ok() {
        return Err(format!("Помилка geocoding: {}", response.status()));
    }

    let geo_response: GeocodingResponse = response
        .json()
        .await
        .map_err(|e| format!("Помилка парсингу: {}", e))?;

    geo_response
        .results
        .and_then(|mut r| r.pop())
        .ok_or_else(|| "Місто не знайдено".to_string())
}

async fn get_weather_by_coords(lat: f64, lon: f64) -> Result<WeatherResponse, String> {
    let url = format!(
        "{}?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,apparent_temperature,weather_code,wind_speed_10m,surface_pressure",
        WEATHER_API, lat, lon
    );

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Помилка мережі: {}", e))?;

    if !response.ok() {
        return Err(format!("Помилка погоди: {}", response.status()));
    }

    response
        .json::<WeatherResponse>()
        .await
        .map_err(|e| format!("Помилка парсингу даних: {}", e))
}

pub async fn get_weather(city: &str) -> Result<WeatherData, String> {
    // Отримуємо координати міста
    let location = get_coordinates(city).await?;
    
    // Отримуємо погоду за координатами
    let weather = get_weather_by_coords(location.latitude, location.longitude).await?;
    
    let (description, icon) = get_weather_description(weather.current.weather_code);
    
    Ok(WeatherData {
        name: location.name.clone(),
        country: location.country.clone(),
        main: MainWeather {
            temp: weather.current.temperature_2m,
            feels_like: weather.current.apparent_temperature,
            humidity: weather.current.relative_humidity_2m,
            pressure: weather.current.surface_pressure as i32,
        },
        weather: WeatherInfo {
            description: description.to_string(),
            icon: icon.to_string(),
        },
        wind: Wind {
            speed: weather.current.wind_speed_10m / 3.6, // конвертуємо км/год в м/с
        },
        sys: Sys {
            country: location.country,
        },
        visibility: 10, // Open-Meteo не надає видимість, ставимо за замовчуванням 10 км
    })
}
