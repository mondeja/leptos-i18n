use leptos::document;

pub fn get_cookie_value(cookie_name: &str) -> Option<String> {
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

pub fn set_cookie_value(cookie_name: &str, value: &str, attributes: &str) {
    use wasm_bindgen::JsCast;

    let doc = document().unchecked_into::<web_sys::HtmlDocument>();

    // Note that we set the cookie to expire in 10 seconds
    // Also that Secure works at localhost even when without https: scheme
    let cookie = format!("{}={};{}", cookie_name, value, attributes);
    doc.set_cookie(&cookie).unwrap();
}
