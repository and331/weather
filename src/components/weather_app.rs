use leptos::*;
use crate::api::weather::*;
use wasm_bindgen::prelude::*;

// Інтеграція з Lucide icons
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = lucide)]
    fn createIcons();
}

// Функція для отримання поточної години
#[wasm_bindgen(inline_js = "export function get_current_hour() { return new Date().getHours(); }")]
extern "C" {
    fn get_current_hour() -> i32;
}

#[component]
pub fn WeatherApp() -> impl IntoView {
    let (city, set_city) = create_signal(String::new());
    let (weather_data, set_weather_data) = create_signal(None::<WeatherData>);
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    let (selected_day, set_selected_day) = create_signal(0);

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
                    
                    // Ініціалізуємо Lucide icons після оновлення DOM
                    request_animation_frame(move || {
                        createIcons();
                    });
                }
                Err(e) => {
                    set_error.set(Some(format!("Помилка: {}", e)));
                    set_weather_data.set(None);
                }
            }
            set_loading.set(false);
        });
    };

    // Ініціалізуємо ікони при першому рендері
    create_effect(move |_| {
        request_animation_frame(move || {
            createIcons();
        });
    });

    view! {
        <div class="p-4 md:p-8 max-w-6xl mx-auto">
            <header class="flex flex-col md:flex-row md:items-center justify-between gap-4 mb-10">
                <div class="flex items-center gap-2">
                    <div class="bg-blue-400 p-2 rounded-xl text-black">
                        <i data-lucide="thermometer-sun"></i>
                    </div>
                    <h1 class="text-2xl font-bold tracking-tight">"weather"</h1>
                </div>

                <div class="flex-1 max-w-xl relative">
                    <div class="search-bar flex items-center px-5 py-3 gap-3">
                        <i data-lucide="search" class="w-5 h-5 text-gray-400"></i>
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
                            on:click=move |_| { set_city.set("Оржиця".to_string()); do_fetch(); }
                        >"Оржиця"</span>
                        <span 
                            class="hover:text-blue-300 cursor-pointer"
                            on:click=move |_| { set_city.set("Черкаси".to_string()); do_fetch(); }
                        >"Черкаси"</span>
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
                            <MainSection 
                                city=data.name.clone() 
                                region=data.country.clone()
                                selected_day=selected_day
                                set_selected_day=set_selected_day
                            />
                            <WeeklyStrip 
                                selected_day=selected_day
                                set_selected_day=set_selected_day
                                forecast=data.forecast.clone()
                            />
                            <DetailedCard 
                                data=data.clone()
                                selected_day=selected_day
                                forecast=data.forecast.clone()
                            />
                            <DescriptionsInfo city=data.name.clone()/>
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
fn MainSection(
    city: String,
    region: String,
    selected_day: ReadSignal<usize>,
    set_selected_day: WriteSignal<usize>,
) -> impl IntoView {
    view! {
        <section class="mb-8 flex flex-col md:flex-row justify-between items-end gap-6">
            <div>
                <h2 class="text-4xl font-medium mb-1">{format!("Погода у {}", city)}</h2>
                <p class="text-gray-400">{region}</p>
            </div>
            /*<div class="bg-[#2D2F31] p-1 rounded-full flex">
                <button 
                    class="px-6 py-2 rounded-full font-medium text-sm transition-colors"
                    class:bg-blue-200=move || selected_day.get() == 0
                    class:text-black=move || selected_day.get() == 0
                    class:text-gray-400=move || selected_day.get() != 0
                    class:hover:text-white=move || selected_day.get() != 0
                    on:click=move |_| set_selected_day.set(0)
                >
                    "Тиждень"
                </button>
                <button 
                    class="px-6 py-2 rounded-full font-medium text-sm transition-colors"
                    class:bg-blue-200=move || selected_day.get() == 1
                    class:text-black=move || selected_day.get() == 1
                    class:text-gray-400=move || selected_day.get() != 1
                    class:hover:text-white=move || selected_day.get() != 1
                    on:click=move |_| set_selected_day.set(1)
                >
                    "10 днів"
                </button>
            </div>*/
        </section>
    }
}

#[component]
fn WeeklyStrip(
    selected_day: ReadSignal<usize>,
    set_selected_day: WriteSignal<usize>,
    forecast: Option<Vec<crate::api::weather::DayForecast>>,
) -> impl IntoView {
    view! {
        <section class="flex gap-3 overflow-x-auto no-scrollbar mb-8 pb-2">
            {move || {
                if let Some(ref days) = forecast {
                    days.iter().enumerate().map(|(idx, day)| {
                        let day_clone = day.clone();
                        view! {
                            <div 
                                class="m3-card min-w-[140px] p-5 flex flex-col items-center text-center cursor-pointer hover:bg-[#252729] transition-colors"
                                class:m3-card-active=move || selected_day.get() == idx
                                on:click=move |_| {
                                    set_selected_day.set(idx);
                                    // Ініціалізуємо Lucide icons після зміни дня
                                    request_animation_frame(move || {
                                        createIcons();
                                    });
                                }
                            >
                                <span 
                                    class="text-sm font-medium mb-3"
                                    class:text-blue-200=move || selected_day.get() == idx
                                    class:text-gray-400=move || selected_day.get() != idx
                                >
                                    {day_clone.day_name.clone()}
                                </span>
                                <i data-lucide={day_clone.icon.clone()} class={format!("w-10 h-10 mb-3 {}", day_clone.icon_color)}></i>
                                <div class="flex gap-2">
                                    <span class="text-lg font-bold">{format!("{:+}°", day_clone.temp_min)}</span>
                                    <span class="text-lg text-gray-400">{format!("{:+}°", day_clone.temp_max)}</span>
                                </div>
                            </div>
                        }.into_view()
                    }).collect::<Vec<_>>()
                } else {
                    vec![view! {
                        <div class="text-gray-400">"Завантаження прогнозу..."</div>
                    }.into_view()]
                }
            }}
        </section>
    }
}

#[component]
fn DetailedCard(
    data: WeatherData,
    selected_day: ReadSignal<usize>,
    forecast: Option<Vec<crate::api::weather::DayForecast>>,
) -> impl IntoView {
    let hourly_times = vec!["0:00", "3:00", "9:00", "12:00", "15:00", "18:00", "21:00"];

    // Визначаємо індекс поточного часу
    let get_current_hour_index = move || -> usize {
        let hour = get_current_hour();
        match hour {
            0..=2 => 0,   // 0:00
            3..=8 => 1,   // 3:00
            9..=11 => 2,  // 9:00
            12..=14 => 3, // 12:00
            15..=17 => 4, // 15:00
            18..=20 => 5, // 18:00
            _ => 6,       // 21:00
        }
    };

    // Ініціалізуємо іконки після зміни дня
    create_effect(move |_| {
        selected_day.get(); // Відстежуємо зміни
        request_animation_frame(move || {
            createIcons();
        });
    });

    view! {
        <section class="m3-card p-6 md:p-10 mb-8">
            {move || {
                let idx = selected_day.get();
                
                if let Some(ref days) = forecast {
                    if let Some(day) = days.get(idx) {
                        let day_name = day.day_name.clone();
                        let day_icon = day.icon.clone();
                        let icon_color = day.icon_color.clone();
                        let hourly_temps = day.hourly_temps.clone();
                        let hourly_feels = day.hourly_feels.clone();
                        let hourly_pressure = day.hourly_pressure.clone();
                        let hourly_humidity = day.hourly_humidity.clone();
                        let hourly_wind = day.hourly_wind.clone();
                        let sunrise = day.sunrise.clone();
                        let sunset = day.sunset.clone();
                        
                        view! {
                            <div class="grid grid-cols-1 lg:grid-cols-12 gap-10">
                                <div class="lg:col-span-4 flex flex-col justify-center border-b lg:border-b-0 lg:border-r border-[#43474E] pb-8 lg:pb-0 lg:pr-8">
                                    <div class="flex items-center gap-6 mb-6">
                                        <i data-lucide={day_icon} class={format!("w-24 h-24 {}", icon_color)}></i>
                                        <div>
                                            <p class="text-gray-400">{day_name}</p>
                                            <span class="text-7xl font-bold tracking-tighter">{format!("{:.0}°C", data.main.temp)}</span>
                                        </div>
                                    </div>
                                    <div class="space-y-3">
                                        <div class="flex items-center gap-3 text-gray-300">
                                            <i data-lucide="sunrise" class="w-5 h-5 text-orange-300"></i>
                                            <span>{format!("Схід: {}", sunrise.unwrap_or_else(|| "Невідомо".to_string()))}</span>
                                        </div>
                                        <div class="flex items-center gap-3 text-gray-300">
                                            <i data-lucide="sunset" class="w-5 h-5 text-purple-300"></i>
                                            <span>{format!("Захід: {}", sunset.unwrap_or_else(|| "Невідомо".to_string()))}</span>
                                        </div>
                                    </div>
                                </div>

                                <div class="lg:col-span-8 overflow-x-auto no-scrollbar">
                                    <table class="w-full text-left">
                                        <thead>
                                            <tr class="text-gray-400 text-sm border-b border-[#43474E]">
                                                <th class="py-4 font-normal min-w-[120px]">"Показник"</th>
                                                {hourly_times.iter().enumerate().map(|(idx, time)| {
                                                    let current_idx = get_current_hour_index();
                                                    let is_today = selected_day.get() == 0;
                                                    let is_current = is_today && idx == current_idx;
                                                    view! {
                                                        <th 
                                                            class="py-4 font-normal text-center"
                                                            class:bg-opacity-10=move || is_current
                                                            class:text-blue-200=move || is_current
                                                            class=("bg-[#D1E4FF]", move || is_current)
                                                        >
                                                            {*time}
                                                        </th>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </tr>
                                        </thead>
                                        <tbody class="text-sm">
                                            <tr class="border-b border-[#333537]">
                                                <td class="py-4 text-gray-300">"Температура, °C"</td>
                                                {hourly_temps.iter().enumerate().map(|(idx, temp)| {
                                                    let current_idx = get_current_hour_index();
                                                    let is_today = selected_day.get() == 0;
                                                    let is_current = is_today && idx == current_idx;
                                                    view! {
                                                        <td 
                                                            class="text-center"
                                                            class:font-bold=move || is_current
                                                            class:text-white=move || is_current
                                                            class:bg-opacity-10=move || is_current
                                                            class=("bg-[#D1E4FF]", move || is_current)
                                                        >
                                                            {format!("{:+}°", temp)}
                                                        </td>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </tr>
                                            <tr class="border-b border-[#333537]">
                                                <td class="py-4 text-gray-300">"Відчувається як"</td>
                                                {hourly_feels.iter().enumerate().map(|(idx, feels)| {
                                                    let current_idx = get_current_hour_index();
                                                    let is_today = selected_day.get() == 0;
                                                    let is_current = is_today && idx == current_idx;
                                                    view! {
                                                        <td 
                                                            class="text-center"
                                                            class:bg-opacity-10=move || is_current
                                                            class=("bg-[#D1E4FF]", move || is_current)
                                                        >
                                                            {format!("{:+}°", feels)}
                                                        </td>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </tr>
                                            <tr class="border-b border-[#333537]">
                                                <td class="py-4 text-gray-300">"Тиск, мм"</td>
                                                {hourly_pressure.iter().enumerate().map(|(idx, pressure)| {
                                                    let p = *pressure;
                                                    let current_idx = get_current_hour_index();
                                                    let is_today = selected_day.get() == 0;
                                                    let is_current = is_today && idx == current_idx;
                                                    view! {
                                                        <td 
                                                            class="text-center"
                                                            class:bg-opacity-10=move || is_current
                                                            class=("bg-[#D1E4FF]", move || is_current)
                                                        >
                                                            {p}
                                                        </td>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </tr>
                                            <tr class="border-b border-[#333537]">
                                                <td class="py-4 text-gray-300">"Вологість, %"</td>
                                                {hourly_humidity.iter().enumerate().map(|(idx, humidity)| {
                                                    let h = *humidity;
                                                    let current_idx = get_current_hour_index();
                                                    let is_today = selected_day.get() == 0;
                                                    let is_current = is_today && idx == current_idx;
                                                    view! {
                                                        <td 
                                                            class="text-center"
                                                            class:bg-opacity-10=move || is_current
                                                            class=("bg-[#D1E4FF]", move || is_current)
                                                        >
                                                            {h}
                                                        </td>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </tr>
                                            <tr>
                                                <td class="py-4 text-gray-300">"Вітер, м/с"</td>
                                                {hourly_wind.iter().enumerate().map(|(idx, wind)| {
                                                    let w = wind.clone();
                                                    let current_idx = get_current_hour_index();
                                                    let is_today = selected_day.get() == 0;
                                                    let is_current = is_today && idx == current_idx;
                                                    view! {
                                                        <td 
                                                            class="text-center"
                                                            class:bg-opacity-10=move || is_current
                                                            class=("bg-[#D1E4FF]", move || is_current)
                                                        >
                                                            {w}
                                                        </td>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </tr>
                                        </tbody>
                                    </table>
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="text-center py-10 text-gray-400">
                                "Дані для цього дня не знайдено"
                            </div>
                        }.into_view()
                    }
                } else {
                    view! {
                        <div class="text-center py-10 text-gray-400">
                            "Завантаження деталей прогнозу..."
                        </div>
                    }.into_view()
                }
            }}
        </section>
    }
}

#[component]
fn DescriptionsInfo(city: String) -> impl IntoView {
    view! {
        /*<div class="grid grid-cols-1 md:grid-cols-2 gap-8 items-start">
            <div class="space-y-6">
                <div class="bg-[#1A1C1E] p-8 rounded-[28px]">
                    <h3 class="text-xl font-medium mb-4 text-blue-200 flex items-center gap-2">
                        <i data-lucide="info" class="w-5 h-5"></i>
                        " Опис погоди"
                    </h3>
                    <p class="text-gray-400 leading-relaxed">
                        {format!("До самого вечора у {} буде триматися похмура погода, лише під кінець дня небо очиститься від хмар. Без опадів. Вечір обіцяє бути прохолодним, але тихим.", city)}
                    </p>
                </div>

                <div class="bg-[#1A1C1E] p-8 rounded-[28px]">
                    <h3 class="text-xl font-medium mb-4 text-green-200 flex items-center gap-2">
                        <i data-lucide="users" class="w-5 h-5"></i>
                        " Народний прогноз"
                    </h3>
                    <p class="text-gray-400 leading-relaxed italic">
                        "\"Якщо швидко танув сніг, то сінокіс обіцяв бути хорошим. У давнину говорили, що 28 лютого зима з весною починає боротися...\" "
                        <a href="#" class="text-blue-300 underline ml-1 font-medium">"Детальніше"</a>
                    </p>
                </div>
            </div>

            <div class="space-y-6">
                <div class="bg-[#1A1C1E] p-8 rounded-[28px]">
                    <div class="flex justify-between items-center mb-6">
                        <h3 class="text-xl font-medium text-red-200 flex items-center gap-2">
                            <i data-lucide="history" class="w-5 h-5"></i>
                            " Історія (за 130 років)"
                        </h3>
                        <span class="text-xs text-gray-500 uppercase font-bold tracking-widest">"28 лютого"</span>
                    </div>
                    <div class="space-y-4">
                        <div class="flex justify-between items-center p-4 bg-[#2D2F31] rounded-2xl">
                            <span class="text-gray-400">"Максимальна:"</span>
                            <div class="text-right">
                                <span class="text-xl font-bold text-red-400">"+12°C"</span>
                                <p class="text-xs text-gray-500">"1998 рік"</p>
                            </div>
                        </div>
                        <div class="flex justify-between items-center p-4 bg-[#2D2F31] rounded-2xl">
                            <span class="text-gray-400">"Мінімальна:"</span>
                            <div class="text-right">
                                <span class="text-xl font-bold text-blue-400">"-22°C"</span>
                                <p class="text-xs text-gray-500">"1986 рік"</p>
                            </div>
                        </div>
                    </div>
                </div>

                <button class="w-full bg-[#D1E4FF] hover:bg-[#B3D1FF] text-[#003258] font-bold py-4 rounded-[28px] transition-all flex items-center justify-center gap-2 shadow-lg">
                    <i data-lucide="share-2" class="w-5 h-5"></i>
                    " Поділитися прогнозом"
                </button>
            </div>
        </div>*/
    }
}

#[component]
fn WelcomeScreen() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center py-20 text-center">
            <div class="mb-6">
                <i data-lucide="cloud-sun" class="w-32 h-32 text-blue-400 inline-block"></i>
            </div>
            <h2 class="text-3xl font-bold mb-4">"Ласкаво просимо"</h2>
            <p class="text-gray-400 text-lg max-w-md">
                "Введіть назву міста, щоб отримати детальний прогноз погоди"
            </p>
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="mt-16 text-center text-gray-600 text-sm pb-8">
            <p>"© 2026 weather. Дані оновлюються кожні 15 хвилин."</p>
        </footer>
    }
}

// Допоміжна функція для request_animation_frame
fn request_animation_frame(f: impl Fn() + 'static) {
    use wasm_bindgen::JsCast;
    let window = web_sys::window().expect("no global window exists");
    let closure = wasm_bindgen::closure::Closure::once(Box::new(f) as Box<dyn FnOnce()>);
    window
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .expect("should register animation frame");
    closure.forget();
}
