use fireplace::{config::Link, devices::Device, eventhandler::EventType};
use gloo_console::log;
use gloo_net::http::Request;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_hooks::prelude::*;
pub mod components;
mod utils;
mod views;

use crate::utils::get_rest;
use views::DeviceList;

#[function_component(App)]
fn app() -> Html {
    let devices = use_state(|| vec![]);
    let links = use_state(|| vec![]);
    let version = use_state(|| String::new());
    {
        let devices = devices.clone();
        let links = links.clone();
        let version = version.clone();
        use_interval(
            move || {
                let devices = devices.clone();
                let links = links.clone();
                let version = version.clone();
                wasm_bindgen_futures::spawn_local({
                    get_rest("/api/devices", move |data| {
                        match serde_json::from_str(data) {
                            Ok(res) => {
                                let mut fetched_videos: Vec<Device> = res;
                                fetched_videos.sort_by(|a, b| b.id.cmp(&a.id));
                                devices.set(fetched_videos);
                            }
                            Err(err) => {
                                log!("Error at decode", JsValue::from(err.to_string()));
                            }
                        }
                    })
                });
                wasm_bindgen_futures::spawn_local({
                    get_rest("/api/links", move |data| match serde_json::from_str(data) {
                        Ok(res) => {
                            let fetched_links: Vec<Link> = res;
                            links.set(fetched_links)
                        }
                        Err(err) => {
                            log!("Error at decode", JsValue::from(err.to_string()));
                        }
                    })
                });
                wasm_bindgen_futures::spawn_local({
                    get_rest("/api/version", move |data| version.set(data.to_string()))
                });
            },
            500,
        );
    }

    let links: Html = links
        .iter()
        .map(|link| html!(<li><a href={link.address.clone()}>{&link.name}</a></li>))
        .collect();

    let action_closure = |event: EventType| {
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(req) = Request::post("/api/trigger-action").json(&event) {
                let _ = req.send().await;
            }
        });
    };

    html! {
        <>
        <header>
            <div id={"header_left"}><img src="logo.svg" height="60"/></div>
            <div id={"header_middle"}> {"Fireplace"} </div>
            <div id={"header_right"}>{&*version}</div>
        </header>
        <nav class={"App__nav"}>
            <ul>
                <li>
                    <a href={"/"} class={"router-link-active router-link-exact-active"} aria-current={"page"}>{"Home"}</a>
                </li>
                {links}
            </ul>
        </nav>
        <DeviceList devices={(*devices).clone()} on_click={action_closure}/>
        <footer> {"?? Pingoin-Tech"} </footer>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
