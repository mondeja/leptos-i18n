use std::str::FromStr;

use leptos::*;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(Clone, EnumIter, EnumString, Display, Default, PartialEq)]
enum Language {
    #[default]
    English,
    #[strum(serialize = "Español")]
    Spanish,
}

#[component]
fn I18nPage(cx: Scope) -> impl IntoView {
    let (language, set_language) = create_signal::<Language>(cx, Language::default());

    view! { cx,
        // Show the title translated
        <h1>{move || {
            match language.get() {
                Language::English => "Welcome to Leptos!",
                Language::Spanish => "¡Bienvenido a Leptos!",
            }
        }}</h1>

        // Show the language translated
        <p>{move || {
            match language.get() {
                Language::English => "Language: ",
                Language::Spanish => "Idioma: ",
            }
        }}
        <strong>{move || language.get().to_string()}</strong></p>
        
        // Handle language selection
        <select on:change=move |ev| {
            let val = event_target_value(&ev);
            set_language.update(|lang| { *lang = Language::from_str(&val).unwrap(); });
        }>
            {
                Language::iter().map(
                    |lang| view! { cx,
                        <option selected={lang == language.get()}>
                            {lang.to_string()}
                        </option>
                    }
                ).collect::<Vec<_>>()
            }
        </select>
    }
}

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        view! { cx,
            <I18nPage/>
        }
    })
}
