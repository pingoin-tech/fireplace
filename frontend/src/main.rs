use fireplace::{devices::Device, eventhandler::EventType};
use gloo_console::log;
use gloo_net::http::Request;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_hooks::prelude::*;
mod device_list;
use device_list::DeviceList;

#[function_component(App)]
fn app() -> Html {
    let devices = use_state(|| vec![]);
    {
        let devices = devices.clone();
        use_interval(
            move || {
                let devices = devices.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match Request::get("/api/devices").send().await {
                        Ok(result) => match result.json().await {
                            Ok(res) => {
                                let mut fetched_videos: Vec<Device> = res;
                                fetched_videos.sort_by(|a, b| b.id.cmp(&a.id));
                                devices.set(fetched_videos);
                            }
                            Err(err) => {
                                log!("Error at decode", JsValue::from(err.to_string()));
                            }
                        },
                        Err(err) => {
                            log!("Error at fetch", JsValue::from(err.to_string()));
                        }
                    }
                });
            },
            500,
        );
    }

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
            <div id={"header_left"}></div>
            <div id={"header_middle"}> {"Fireplace"} </div>
            <div id={"header_right"}><img src="logo.svg" height="60"/></div>
        </header>
        <nav class={"App__nav"}>
            <ul>
                <li>
                    <a href={"/"} class={"router-link-active router-link-exact-active"} aria-current={"page"}>{"Home"}</a>
                </li>
            </ul>
        </nav>
        <main>
        <article>

        <h1>{"Devices"}</h1>
        <table>
          <thead>
            <tr>
              <th>{"id"}</th>
              <th>{"ip/mac"}</th>
              <th>{"RSSI"}</th>
              <th>{"sub-device"}</th>
              <th>{"actions"}</th>
            </tr>
          </thead>
          <tbody>
            <DeviceList devices={(*devices).clone()} on_click={action_closure}/>
          </tbody>
        </table>
      </article>
      </main>
      <footer> {"© Pingoin-Tech"} </footer>
      </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}