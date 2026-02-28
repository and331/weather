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
        <div class="p-4 md:p-8 max-w-6xl mx-auto">
            <header class="flex flex-col md:flex-row md:items-center justify-between gap-4 mb-10">
                <div class="flex items-center gap-2">
                    <div class="bg-blue-400 p-2 rounded-xl text-black text-2xl">
                        "🌤️"
                    </div>
                    <h1 class="text-2xl font-bold tracking-tight">"синоптик"</h1>
                </div>

                <div class="flex-1 max-w-xl relative">
                    <div class="search-bar flex items-center px-5 py-3 gap-3">
                        <span class="text-gray-400 text-xl">"🔍"</span>
                        <input 
                            type="text" 
                            placeholder="Назва населеного пункту..." 
                            class="bg-transparent border-none outline-none w-full text-white placeholder-gray-500"
                            on:input=move |ev| {
                                set_city.set(event_target_value(&ev));
                            }
                            on:keypress=move |ev: web_sys::KeyboardEvent| {
                                if ev.key() == "Enter" {
                                    do_fetch();
                                }
                            }
                        />
                        <button 
                            class="bg-blue-400 hover:bg-blue-300 text-black font-medium px-6 py-2 rounded-full transition-colors"
                            on:click=move |_| do_fetch()
                        >
                            "Пошук"
                        </button>
                    </div>
                    <div class="flex gap-4 mt-2 px-2 text-sm text-gray-400">
                        <span 
                            class="hover:text-blue-300 cursor-pointer"
                            on:click=move |_| { set_city.set("Київ".to_string()); do_fetch(); }
                        >"Київ"</span>
                        <span 
                            class="hover:text-blue-300 cursor-pointer"
                            on:click=move |_| { set_city.set("Черкаси".to_string()); do_fetch(); }
                        >"Черкаси"</span>
                        <span 
                            class="hover:text-blue-300 cursor-pointer"
                            on:click=move |_| { set_city.set("Львів".to_string()); do_fetch(); }
                        >"Львів"</span>
                    </div>
                </div>
            </header>
            
            {move || error.get().map(|err| {
                view! {
                    <div class="bg-red-900 bg-opacity-20 border border-red-700 text-red-300 px-6 py-4 rounded-[28px] mb-8">
                        {err}
                    </div>
                }
            })}

            {move || {
                if loading.get() {
                    view! {
                        <div class="flex flex-col items-center justify-center py-20">
                            <div class="spinner mb-4"></div>
                            <p class="text-gray-400">"Завантаження..."</p>
                        </div>
                    }.into_view()
                } else if let Some(data) = weather_data.get() {
                    view! {
                        <>
                            <CityTitle city=data.name.clone() country=data.country.clone()/>
                            <MainWeatherCard data=data.clone()/>
                            <WeatherDetails data=data/>
                        </>
                    }.into_view()
                } else {
                    view! {
                        <WelcomeScreen/>
                    }.into_view()
                }
            }}
            
            <Footer/>
        </div>
    }
}

#[component]
fn CityTitle(city: String, country: String) -> impl IntoView {
    view! {
        <section class="mb-8">
            <h2 class="text-4xl font-medium mb-1">{format!("Погода у {}", city)}</h2>
            <p class="text-gray-400">{country}</p>
        </section>
    }
}

#[component]
fn MainWeatherCard(data: WeatherData) -> impl IntoView {
    let weather_emoji = get_weather_emoji(&data.weather.icon);
    
    view! {
        <section class="m3-card p-6 md:p-10 mb-8">
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-10">
                <div class="flex flex-col justify-center border-b lg:border-b-0 lg:border-r border-gray-700 pb-8 lg:pb-0 lg:pr-8">
                    <div class="flex items-center gap-6 mb-6">
                        <div class="text-8xl">{weather_emoji}</div>
                        <div>
                            <p class="text-gray-400 mb-2">{data.weather.description.clone()}</p>
                            <span class="text-7xl font-bold tracking-tighter">{format!("{:.1}°C", data.main.temp)}</span>
                        </div>
                    </div>
                    <div class="space-y-3">
                        <div class="flex items-center gap-3 text-gray-300">
                            <span class="text-orange-300">"🌡️"</span>
                            <span>{format!("Відчувається як {:.1}°C", data.main.feels_like)}</span>
                        </div>
                    </div>
                </div>

                <div class="space-y-4">
                    <WeatherMetric 
                        label="Вологість".to_string() 
                        value=format!("{}%", data.main.humidity)
                        icon="💧".to_string()
                    />
                    <WeatherMetric 
                        label="Вітер".to_string() 
                        value=format!("{:.1} м/с", data.wind.speed)
                        icon="🌬️".to_string()
                    />
                    <WeatherMetric 
                        label="Тиск".to_string() 
                        value=format!("{} гПа", data.main.pressure)
                        icon="🌡️".to_string()
                    />
                    <WeatherMetric 
                        label="Видимість".to_string() 
                        value=format!("{} км", data.visibility)
                        icon="👁️".to_string()
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn WeatherMetric(label: String, value: String, icon: String) -> impl IntoView {
    view! {
        <div class="flex justify-between items-center p-4 bg-[#2D2F31] rounded-2xl">
            <span class="text-gray-400 flex items-center gap-2">
                <span>{icon}</span>
                {label}
            </span>
            <span class="text-xl font-bold text-white">{value}</span>
        </div>
    }
}

#[component]
fn WeatherDetails(data: WeatherData) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
            <div class="bg-[#1A1C1E] p-8 rounded-[28px]">
                <h3 class="text-xl font-medium mb-4 text-blue-200 flex items-center gap-2">
                    <span>"ℹ️"</span>
                    " Інформація"
                </h3>
                <div class="space-y-3 text-gray-300">
                    <p><strong>"Місто:"</strong> " " {data.name}</p>
                    <p><strong>"Країна:"</strong> " " {data.sys.country}</p>
                    <p><strong>"Температура:"</strong> " " {format!("{:.1}°C", data.main.temp)}</p>
                    <p><strong>"Опис:"</strong> " " {data.weather.description}</p>
                </div>
            </div>

            <div class="bg-[#1A1C1E] p-8 rounded-[28px]">
                <h3 class="text-xl font-medium mb-4 text-green-200 flex items-center gap-2">
                    <span>"📊"</span>
                    " Джерело даних"
                </h3>
                <div class="space-y-3 text-gray-300">
                    <p>"Дані надані Open-Meteo"</p>
                    <p class="text-sm text-gray-500">"Безкоштовний API для отримання погодних даних без необхідності реєстрації."</p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn WelcomeScreen() -> impl IntoView {
    view! {
        <div class="m3-card p-12 text-center">
            <div class="text-8xl mb-6">"🌍"</div>
            <h2 class="text-3xl font-bold mb-4">"Вітаємо у Синоптику!"</h2>
            <p class="text-gray-400 text-lg max-w-2xl mx-auto">
                "Введіть назву міста у пошуковому полі вгорі, щоб дізнатися поточну погоду. 
                Дані надаються Open-Meteo API."
            </p>
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="mt-16 text-center text-gray-600 text-sm pb-8">
            <p>"© 2026 Синоптик. Дизайн у стилі Material You. Дані від Open-Meteo."</p>
        </footer>
    }
}

fn get_weather_emoji(icon: &str) -> &'static str {
    match icon {
        "01d" => "☀️",
        "02d" => "🌤️",
        "03d" => "☁️",
        "04d" => "☁️",
        "09d" => "🌧️",
        "10d" => "🌦️",
        "11d" => "⛈️",
        "13d" => "❄️",
        "50d" => "🌫️",
        _ => "🌤️",
    }
}
