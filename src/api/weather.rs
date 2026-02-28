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

// Структура для 7-денного прогнозу
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastResponse {
    pub daily: DailyForecast,
    pub hourly: HourlyForecast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyForecast {
    pub time: Vec<String>,
    pub temperature_2m_max: Vec<f64>,
    pub temperature_2m_min: Vec<f64>,
    pub weather_code: Vec<i32>,
    pub sunrise: Vec<String>,
    pub sunset: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlyForecast {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
    pub apparent_temperature: Vec<f64>,
    pub relative_humidity_2m: Vec<i32>,
    pub surface_pressure: Vec<f64>,
    pub wind_speed_10m: Vec<f64>,
    pub wind_direction_10m: Vec<i32>,
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
    pub forecast: Option<Vec<DayForecast>>,
}

// Прогноз на один день
#[derive(Debug, Clone)]
pub struct DayForecast {
    pub date: String,
    pub day_name: String,
    pub icon: String,
    pub icon_color: String,
    pub temp_min: i32,
    pub temp_max: i32,
    pub hourly_temps: Vec<i32>,
    pub hourly_feels: Vec<i32>,
    pub hourly_pressure: Vec<i32>,
    pub hourly_humidity: Vec<i32>,
    pub hourly_wind: Vec<String>,
    pub sunrise: Option<String>,
    pub sunset: Option<String>,
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

// Функція для отримання Lucide іконки за кодом WMO
fn get_weather_icon(code: i32) -> (&'static str, &'static str) {
    match code {
        0 => ("sun", "text-yellow-400"),
        1 | 2 => ("cloud-sun", "text-yellow-200"),
        3 => ("cloud", "text-gray-400"),
        45 | 48 => ("cloud-fog", "text-gray-300"),
        51 | 53 | 55 => ("cloud-drizzle", "text-blue-300"),
        56 | 57 => ("snowflake", "text-blue-200"),
        61 | 63 | 65 => ("cloud-rain", "text-blue-400"),
        66 | 67 => ("cloud-snow", "text-white"),
        71 | 73 | 75 => ("cloud-snow", "text-white"),
        77 => ("cloud-hail", "text-blue-100"),
        80 | 81 | 82 => ("cloud-rain-wind", "text-blue-500"),
        85 | 86 => ("snowflake", "text-white"),
        95 => ("cloud-lightning", "text-purple-400"),
        96 | 99 => ("cloud-lightning", "text-purple-500"),
        _ => ("cloud", "text-gray-400"),
    }
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

// Функція для отримання напрямку вітру
fn get_wind_direction(degrees: i32) -> &'static str {
    match degrees {
        0..=22 | 338..=360 => "↑",
        23..=67 => "↗",
        68..=112 => "→",
        113..=157 => "↘",
        158..=202 => "↓",
        203..=247 => "↙",
        248..=292 => "←",
        293..=337 => "↖",
        _ => "→",
    }
}

// Функція для форматування часу з ISO 8601 в HH:MM
fn format_time(datetime: &str) -> String {
    // Формат: "2026-02-28T06:37" -> "06:37"
    if let Some(time_part) = datetime.split('T').nth(1) {
        time_part.split(':').take(2).collect::<Vec<_>>().join(":")
    } else {
        datetime.to_string()
    }
}

// Функція для отримання назви дня тижня українською
fn get_day_name(date_str: &str) -> String {
    // Парсимо дату формату "2026-02-28"
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        return date_str.to_string();
    }
    
    let year = parts[0].parse::<i32>().unwrap_or(2026);
    let month = parts[1].parse::<i32>().unwrap_or(1);
    let day = parts[2].parse::<i32>().unwrap_or(1);
    
    // Алгоритм Зеллера для визначення дня тижня
    let m = if month < 3 { month + 12 } else { month };
    let y = if month < 3 { year - 1 } else { year };
    let k = y % 100;
    let j = y / 100;
    let h = (day + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
    
    let day_names = ["Сб", "Нд", "Пн", "Вт", "Ср", "Чт", "Пт"];
    let day_name = day_names[h as usize % 7];
    
    format!("{} {}/{}", day_name, day, month)
}

async fn get_forecast_by_coords(lat: f64, lon: f64) -> Result<ForecastResponse, String> {
    let url = format!(
        "{}?latitude={}&longitude={}&daily=temperature_2m_max,temperature_2m_min,weather_code,sunrise,sunset&hourly=temperature_2m,apparent_temperature,relative_humidity_2m,surface_pressure,wind_speed_10m,wind_direction_10m&forecast_days=7&timezone=auto",
        WEATHER_API, lat, lon
    );

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Помилка мережі: {}", e))?;

    if !response.ok() {
        return Err(format!("Помилка прогнозу: {}", response.status()));
    }

    response
        .json::<ForecastResponse>()
        .await
        .map_err(|e| format!("Помилка парсингу прогнозу: {}", e))
}

pub async fn get_weather(city: &str) -> Result<WeatherData, String> {
    // Отримуємо координати міста
    let location = get_coordinates(city).await?;
    
    // Отримуємо поточну погоду
    let weather = get_weather_by_coords(location.latitude, location.longitude).await?;
    
    // Отримуємо 7-денний прогноз
    let forecast_result = get_forecast_by_coords(location.latitude, location.longitude).await;
    
    let forecast_days = if let Ok(forecast) = forecast_result {
        let mut days = Vec::new();
        
        for i in 0..7.min(forecast.daily.time.len()) {
            let date = &forecast.daily.time[i];
            let day_name = get_day_name(date);
            let (icon, icon_color) = get_weather_icon(forecast.daily.weather_code[i]);
            
            // Отримуємо погодинні дані для цього дня (кожні 3 години: 0, 3, 9, 12, 15, 18, 21)
            let hours = vec![0, 3, 9, 12, 15, 18, 21];
            let base_hour = i * 24;
            
            let hourly_temps: Vec<i32> = hours.iter()
                .filter_map(|h| forecast.hourly.temperature_2m.get(base_hour + h).map(|&t| t as i32))
                .collect();
            
            let hourly_feels: Vec<i32> = hours.iter()
                .filter_map(|h| forecast.hourly.apparent_temperature.get(base_hour + h).map(|&t| t as i32))
                .collect();
            
            let hourly_pressure: Vec<i32> = hours.iter()
                .filter_map(|h| forecast.hourly.surface_pressure.get(base_hour + h).map(|&p| (p * 0.75).round() as i32))
                .collect();
            
            let hourly_humidity: Vec<i32> = hours.iter()
                .filter_map(|h| forecast.hourly.relative_humidity_2m.get(base_hour + h).copied())
                .collect();
            
            let hourly_wind: Vec<String> = hours.iter()
                .filter_map(|h| forecast.hourly.wind_direction_10m.get(base_hour + h).map(|&d| get_wind_direction(d).to_string()))
                .collect();
            
            // Форматуємо час схід/заходу (ISO 8601 -> HH:MM)
            let sunrise = forecast.daily.sunrise.get(i).map(|s| format_time(s));
            let sunset = forecast.daily.sunset.get(i).map(|s| format_time(s));
            
            days.push(DayForecast {
                date: date.clone(),
                day_name,
                icon: icon.to_string(),
                icon_color: icon_color.to_string(),
                temp_min: forecast.daily.temperature_2m_min[i] as i32,
                temp_max: forecast.daily.temperature_2m_max[i] as i32,
                hourly_temps,
                hourly_feels,
                hourly_pressure,
                hourly_humidity,
                hourly_wind,
                sunrise,
                sunset,
            });
        }
        
        Some(days)
    } else {
        None
    };
    
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
            speed: weather.current.wind_speed_10m / 3.6,
        },
        sys: Sys {
            country: location.country,
        },
        visibility: 10,
        forecast: forecast_days,
    })
}
