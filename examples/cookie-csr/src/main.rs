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

fn initial_language_from_cookie() -> Language {
    use wasm_bindgen::JsCast;

    let doc = document().unchecked_into::<web_sys::HtmlDocument>();
    let cookie = doc.cookie().unwrap_or_default();
    cookie
        .replace("%C3%B1", "ñ")
        .split_once('=')
        .map(|c| c.1)
        .map(|lang| Language::from_str(lang).unwrap_or_default())
        .unwrap_or_default()
}

fn set_language_cookie(lang: Language) {
    use wasm_bindgen::JsCast;

    let doc = document().unchecked_into::<web_sys::HtmlDocument>();

    // Very dirty workaround for the ñ character (see the README):
    let encoded_lang = lang.to_string().replace('ñ', "%C3%B1");

    // Note that we set the cookie to expire in 10 seconds
    // Also that Secure works at localhost even when without https: scheme
    let cookie = format!("language={};Max-Age=10;Path=/;Secure", encoded_lang);
    doc.set_cookie(&cookie).unwrap();
}

#[component]
fn I18nPage(cx: Scope) -> impl IntoView {
    let initial_language = initial_language_from_cookie();

    let (language, set_language) = create_signal::<Language>(cx, initial_language);

    view! { cx,
        // Show the title translated
        <h1>{move || {
            match language.get() {
                Language::English => "Welcome to Leptos!",
                Language::Spanish => "¡Bienvenido a Leptos!",
            }
        }}</h1>

        // Show the language translated
        <p>
            {move || {
                match language.get() {
                    Language::English => "Language: ",
                    Language::Spanish => "Idioma: ",
                }
            }}
            {move || language.get().to_string()}
        </p>

        // Handle language selection
        <select on:change=move |ev| {
            let val = event_target_value(&ev);
            // Note that we call `set_language` write signal and we
            // need to update the cookie as well
            set_language.update(|lang| {
                *lang = Language::from_str(&val).unwrap();
                set_language_cookie(lang.clone());
            });
        }>
            {
                move || {Language::iter().map(
                    |lang| view! { cx,
                        <option selected={lang == language.get()}>
                            {lang.to_string()}
                        </option>
                    }
                ).collect::<Vec<_>>()}
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
