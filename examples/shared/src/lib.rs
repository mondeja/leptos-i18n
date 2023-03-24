use leptos::document;

pub fn get_value_of_cookie(cookie_name: &str) -> Option<String> {
    use wasm_bindgen::JsCast;

    let doc = document().unchecked_into::<web_sys::HtmlDocument>();
    let cookies = doc.cookie().unwrap_or_default();
    if cookies.is_empty() {
        return None;
    }

    for cookie in cookies.split(';') {
        let cookie = cookie.trim();
        if cookie.starts_with(&format!("{}=", cookie_name)) {
            let (_, value) = cookie.split_once('=').unwrap();
            return Some(value.to_string());
        }
    }
    None
}
