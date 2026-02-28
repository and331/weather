use leptos::*;
use crate::api::weather::*;
use wasm_bindgen::prelude::*;

// Інтеграція з Lucide icons
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = lucide)]
    fn createIcons();
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
                            />
                            <DetailedCard 
                                data=data.clone()
                                selected_day=selected_day
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
            <div class="bg-[#2D2F31] p-1 rounded-full flex">
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
            </div>
        </section>
    }
}

#[component]
fn WeeklyStrip(
    selected_day: ReadSignal<usize>,
    set_selected_day: WriteSignal<usize>,
) -> impl IntoView {
    let days = vec![
        ("Сб 28", "cloud-sun", -3, 5, "text-yellow-200"),
        ("Нд 29", "sun", -1, 7, "text-yellow-400"),
        ("Пн 30", "cloud-rain", 0, 4, "text-blue-400"),
        ("Вт 01", "cloud-snow", -4, 2, "text-white"),
        ("Ср 02", "cloud", -3, 5, "text-gray-400"),
        ("Чт 03", "cloud-lightning", -6, 4, "text-purple-400"),
        ("Пт 04", "sun", -2, 8, "text-yellow-400"),
    ];

    view! {
        <section class="flex gap-3 overflow-x-auto no-scrollbar mb-8 pb-2">
            {days.into_iter().enumerate().map(|(idx, (date, icon, temp_min, temp_max, color))| {
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
                            {date}
                        </span>
                        <i data-lucide={icon} class={format!("w-10 h-10 mb-3 {}", color)}></i>
                        <div class="flex gap-2">
                            <span class="text-lg font-bold">{format!("{:+}°", temp_min)}</span>
                            <span class="text-lg text-gray-400">{format!("{:+}°", temp_max)}</span>
                        </div>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </section>
    }
}

#[component]
fn DetailedCard(
    data: WeatherData,
    selected_day: ReadSignal<usize>,
) -> impl IntoView {
    // Дані для кожного дня тижня
    let days_data = vec![
        ("Субота, 28 лютого", "cloud-sun", "text-yellow-200", vec![-3, 3, 3, 2, 2, 2, 2], vec![-3, 3, 3, 2, 2, 2, 2], vec![760, 760, 762, 760, 760, 758, 760], vec![59, 32, 35, 69, 69, 59, 99], vec!["↘", "→", "→", "↗", "↗", "↗", "↘"]),
        ("Неділя, 29 лютого", "sun", "text-yellow-400", vec![-1, 0, 4, 7, 6, 4, 1], vec![-2, -1, 3, 7, 5, 3, 0], vec![758, 759, 761, 762, 761, 760, 759], vec![62, 45, 38, 32, 35, 48, 58], vec!["→", "↗", "↗", "↑", "↖", "←", "↙"]),
        ("Понеділок, 01 березня", "cloud-rain", "text-blue-400", vec![0, 1, 2, 4, 3, 2, 1], vec![-1, 0, 1, 3, 2, 1, 0], vec![759, 758, 757, 756, 757, 758, 759], vec![75, 78, 82, 85, 80, 72, 68], vec!["↙", "↓", "↓", "↘", "→", "→", "↗"]),
        ("Вівторок, 01 березня", "cloud-snow", "text-white", vec![-4, -3, -1, 2, 1, -1, -2], vec![-6, -5, -3, 0, -1, -3, -4], vec![762, 763, 764, 765, 764, 763, 762], vec![88, 85, 80, 75, 78, 82, 85], vec!["↑", "↑", "↗", "→", "↘", "↓", "↓"]),
        ("Середа, 02 березня", "cloud", "text-gray-400", vec![-3, -2, 1, 5, 4, 2, 0], vec![-5, -4, -1, 3, 2, 0, -2], vec![761, 760, 761, 762, 761, 760, 759], vec![70, 65, 55, 48, 52, 60, 68], vec!["↙", "←", "↖", "↑", "↗", "→", "→"]),
        ("Четвер, 03 березня", "cloud-lightning", "text-purple-400", vec![-6, -4, -2, 4, 2, 0, -3], vec![-8, -6, -4, 2, 0, -2, -5], vec![757, 756, 755, 754, 755, 757, 758], vec![92, 88, 85, 78, 82, 86, 90], vec!["↓", "↘", "→", "↗", "↑", "↖", "←"]),
        ("П'ятниця, 04 березня", "sun", "text-yellow-400", vec![-2, 0, 3, 8, 7, 5, 2], vec![-3, -1, 2, 7, 6, 4, 1], vec![760, 761, 762, 763, 762, 761, 760], vec![55, 48, 40, 35, 38, 45, 52], vec!["←", "↖", "↑", "↗", "→", "↘", "↓"]),
    ];

    let hourly_times = vec!["0:00", "3:00", "9:00", "12:00", "15:00", "18:00", "21:00"];

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
                let (day_name, day_icon, icon_color, hourly_temps, hourly_feels, hourly_pressure, hourly_humidity, hourly_wind) = 
                    days_data.get(idx).cloned().unwrap_or_else(|| days_data[0].clone());
                
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
                                    <span>"Схід: 06:37"</span>
                                </div>
                                <div class="flex items-center gap-3 text-gray-300">
                                    <i data-lucide="sunset" class="w-5 h-5 text-purple-300"></i>
                                    <span>"Захід: 17:32"</span>
                                </div>
                            </div>
                        </div>

                        <div class="lg:col-span-8 overflow-x-auto no-scrollbar">
                            <table class="w-full text-left">
                                <thead>
                                    <tr class="text-gray-400 text-sm border-b border-[#43474E]">
                                        <th class="py-4 font-normal min-w-[120px]">"Показник"</th>
                                        {hourly_times.iter().enumerate().map(|(idx, time)| {
                                            view! {
                                                <th 
                                                    class="py-4 font-normal text-center"
                                                    class:bg-opacity-10=move || idx == 3
                                                    class:text-blue-200=move || idx == 3
                                                    class=("bg-[#D1E4FF]", move || idx == 3)
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
                                            view! {
                                                <td 
                                                    class="text-center"
                                                    class:font-bold=move || idx == 3
                                                    class:text-white=move || idx == 3
                                                    class:bg-opacity-10=move || idx == 3
                                                    class=("bg-[#D1E4FF]", move || idx == 3)
                                                >
                                                    {format!("{:+}°", temp)}
                                                </td>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </tr>
                                    <tr class="border-b border-[#333537]">
                                        <td class="py-4 text-gray-300">"Відчувається як"</td>
                                        {hourly_feels.iter().enumerate().map(|(idx, feels)| {
                                            view! {
                                                <td 
                                                    class="text-center"
                                                    class:bg-opacity-10=move || idx == 3
                                                    class=("bg-[#D1E4FF]", move || idx == 3)
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
                                            view! {
                                                <td 
                                                    class="text-center"
                                                    class:bg-opacity-10=move || idx == 3
                                                    class=("bg-[#D1E4FF]", move || idx == 3)
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
                                            view! {
                                                <td 
                                                    class="text-center"
                                                    class:bg-opacity-10=move || idx == 3
                                                    class=("bg-[#D1E4FF]", move || idx == 3)
                                                >
                                                    {h}
                                                </td>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </tr>
                                    <tr>
                                        <td class="py-4 text-gray-300">"Вітер, м/с"</td>
                                        {hourly_wind.iter().enumerate().map(|(idx, wind)| {
                                            view! {
                                                <td 
                                                    class="text-center"
                                                    class:bg-opacity-10=move || idx == 3
                                                    class=("bg-[#D1E4FF]", move || idx == 3)
                                                >
                                                    {*wind}
                                                </td>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>
                }
            }}
        </section>
    }
}

#[component]
fn DescriptionsInfo(city: String) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8 items-start">
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
        </div>
    }
}

#[component]
fn WelcomeScreen() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center py-20 text-center">
            <div class="mb-6">
                <i data-lucide="cloud-sun" class="w-32 h-32 text-blue-400 inline-block"></i>
            </div>
            <h2 class="text-3xl font-bold mb-4">"Ласкаво просимо до Синоптика"</h2>
            <p class="text-gray-400 text-lg max-w-md">
                "Введіть назву міста, щоб отримати детальний прогноз погоди з використанням Material You дизайну"
            </p>
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="mt-16 text-center text-gray-600 text-sm pb-8">
            <p>"© 2026 Синоптик. Дизайн у стилі Material You. Дані оновлюються кожні 15 хвилин."</p>
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
