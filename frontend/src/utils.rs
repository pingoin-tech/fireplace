use gloo_net::http::Request;

use crate::Msg;

pub async fn fetch<Fn>(path: &str, fun: Fn) -> Option<Msg>
where
    Fn: FnOnce(String) -> Option<Msg>,
{
    if let Ok(response) = Request::get(path).send().await {
        if !response.ok() {
            None
        } else {
            let response = response.text().await.expect("Decode");
            fun(response)
        }
    } else {
        None
    }
}

pub async fn post<Fn, T>(path: &str, data: T, fun: Fn) -> Option<Msg>
where
    Fn: FnOnce(String) -> Option<Msg>,
    T: serde::Serialize,
{
    if let Ok(request) = Request::post(path).json(&data) {
        if let Ok(response) = request.send().await {
            if !response.ok() {
                None
            } else {
                let response = response.text().await.expect("Decode");
                fun(response)
            }
        } else {
            None
        }
    } else {
        None
    }
}
