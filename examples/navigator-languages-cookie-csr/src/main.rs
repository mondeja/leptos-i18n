use leptos::*;

use shared::{get_cookie_value, set_cookie_value};

#[derive(Clone, PartialEq)]
struct Language {
    code: &'static str,
    name: &'static str,
}

static LANGUAGES: &[Language] = &[
    Language {
        code: "en",
        name: "English",
    },
    Language {
        code: "es",
        name: "Español",
    },
];

impl Default for Language {
    fn default() -> Self {
        Self {
            code: LANGUAGES[0].code,
            name: LANGUAGES[0].name,
        }
    }
}

impl Language {
    fn from_str(s: &str) -> Option<Self> {
        LANGUAGES.iter().find(|l| l.code == s).map(|l| Self {
            code: l.code,
            name: l.name,
        })
    }
}

fn initial_language_from_navigator_languages() -> Option<Language> {
    let languages = web_sys::window().unwrap().navigator().languages().to_vec();
    for raw_language in languages {
        let mut language = raw_language.as_string().expect("Language not parseable");
        if language.contains('-') {
            language = language.split_once('-').unwrap().0.to_string();
        }
        if let Some(lang) = Language::from_str(&language) {
            return Some(lang);
        }
    }
    None
}

fn initial_language_from_cookie_or_navigator_languages() -> Language {
    let initial_language = match get_cookie_value("language") {
        Some(language_cookie) => Language::from_str(&language_cookie),
        None => None,
    };

    match initial_language {
        Some(lang) => lang,
        None => match initial_language_from_navigator_languages() {
            Some(lang) => lang,
            None => Language::default(),
        },
    }
}

fn set_language_cookie(lang: Language) {
    set_cookie_value("language", lang.code, "Max-Age=10;Path=/;Secure");
}

#[component]
fn I18nPage(cx: Scope) -> impl IntoView {
    // Try to get initial language from cookie using `navigator.languages` as fallback
    let initial_language = initial_language_from_cookie_or_navigator_languages();

    let (language, set_language) = create_signal::<Language>(cx, initial_language);

    view! { cx,
        // Show the title translated
        <h1>{move || {
            match language.get().code {
                "en" => "Welcome to Leptos!",
                "es" => "¡Bienvenido a Leptos!",
                &_ => panic!()
            }
        }}</h1>

        // Show the language translated
        <p>
            {move || {
                match language.get().code {
                    "en" => "Language: ",
                    "es" => "Idioma: ",
                    &_ => panic!()
                }
            }}
            {move || language.get().name}
        </p>

        // Handle language selection
        <select on:change=move |ev| {
            let val = event_target_value(&ev);
            set_language.update(|lang| {
                *lang = Language::from_str(&val).unwrap();
                set_language_cookie(lang.clone());
            });
        }>
            {move || {
                LANGUAGES.iter().map(|l| {
                    view! { cx,
                        <option value={l.code} selected={
                            l.code == language.get().code
                        }>{l.name}</option>
                    }
                }).collect::<Vec<_>>()
            }}
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
