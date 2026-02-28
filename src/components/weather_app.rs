use leptos::*;
use crate::api::weather::*;

#[component]
pub fn WeatherApp() -> impl IntoView {
    let (city, set_city) = create_signal(String::new());
    let (weather_data, set_weather_data) = create_signal(None::<WeatherData>);
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);

    let do_fetch = move || {
        let city_value = city.get_untracked();
        if city_value.is_empty() {
            set_error.set(Some("Будь ласка, введіть назву міста".to_string()));
            return;
        }

        set_loading.set(true);
        set_error.set(None);

        spawn_local(async move {
            match get_weather(&city_value).await {
                Ok(data) => {
                    set_weather_data.set(Some(data));
                    set_error.set(None);
                }
                Err(e) => {
                    set_error.set(Some(format!("Помилка: {}", e)));
                    set_weather_data.set(None);
                }
            }
            set_loading.set(false);
        });
    };

    view! {
        <div class="container">
            <header class="header">
                <h1>"🌤️ Погода"</h1>
            </header>

            <div class="search-box">
                <input
                    type="text"
                    placeholder="Введіть назву міста..."
                    class="search-input"
                    on:input=move |ev| {
                        set_city.set(event_target_value(&ev));
                    }
                    on:keypress=move |ev: web_sys::KeyboardEvent| {
                        if ev.key() == "Enter" {
                            do_fetch();
                        }
                    }
                />
                <button class="search-btn" on:click=move |_| do_fetch()>
                    "Знайти"
                </button>
            </div>

            {move || error.get().map(|err| {
                view! {
                    <div class="error-message">
                        {err}
                    </div>
                }
            })}

            {move || {
                if loading.get() {
                    view! {
                        <div class="loading">
                            <div class="spinner"></div>
                            <p>"Завантаження..."</p>
                        </div>
                    }.into_view()
                } else if let Some(data) = weather_data.get() {
                    view! {
                        <WeatherCard data=data/>
                    }.into_view()
                } else {
                    view! {
                        <div class="welcome-message">
                            <p>"👋 Введіть назву міста, щоб побачити погоду"</p>
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}

#[component]
fn WeatherCard(data: WeatherData) -> impl IntoView {
    view! {
        <div class="weather-card">
            <div class="weather-header">
                <h2 class="city-name">{data.name.clone()}</h2>
                <p class="country">{data.sys.country.clone()}</p>
            </div>

            <div class="weather-main">
                <div class="temperature">
                    <span class="temp-value">{format!("{:.1}°C", data.main.temp)}</span>
                    <p class="feels-like">{format!("Відчувається як {:.1}°C", data.main.feels_like)}</p>
                </div>
                <div class="weather-icon">
                    <img
                        src={format!("https://openweathermap.org/img/wn/{}@4x.png", data.weather.icon)}
                        alt={data.weather.description.clone()}
                    />
                    <p class="weather-description">{data.weather.description.clone()}</p>
                </div>
            </div>

            <div class="weather-details">
                <div class="detail-item">
                    <span class="detail-label">"💧 Вологість"</span>
                    <span class="detail-value">{format!("{}%", data.main.humidity)}</span>
                </div>
                <div class="detail-item">
                    <span class="detail-label">"🌬️ Вітер"</span>
                    <span class="detail-value">{format!("{:.1} м/с", data.wind.speed)}</span>
                </div>
                <div class="detail-item">
                    <span class="detail-label">"🌡️ Тиск"</span>
                    <span class="detail-value">{format!("{} гПа", data.main.pressure)}</span>
                </div>
                <div class="detail-item">
                    <span class="detail-label">"👁️ Видимість"</span>
                    <span class="detail-value">{format!("{} км", data.visibility)}</span>
                </div>
            </div>
        </div>
    }
}
