use gloo_console::log;
use gloo_net::http::Request;
use wasm_bindgen::JsValue;

pub async fn get_rest<F>(path: &str, function: F)
where
    F: FnOnce(&str),
{
    match Request::get(path).send().await {
        Ok(result) => match result.text().await {
            Ok(text) => {
                function(text.as_str());
            }
            Err(err) => {
                log!("Error at decode", JsValue::from(err.to_string()));
            }
        },
        Err(err) => {
            log!("Error at fetch", JsValue::from(err.to_string()));
        }
    }
}
