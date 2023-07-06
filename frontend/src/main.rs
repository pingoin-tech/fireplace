use crate::router::{AppRoutes, RouterView};
use fireplace::eventhandler::{EventName, EventType};
use fireplace::{config::Link, devices::Device, eventhandler::Event};
use gloo_timers::future::TimeoutFuture;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore_router::HistoryIntegration;
use sycamore_router::Router;

use serde_json::{self, Error};
mod components;
mod router;
mod utils;
mod views;

use components::{ViewFoot, ViewHead, ViewNav};

use utils::fetch;

// ------ ------
//     Start
// ------ ------

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let version = create_signal(cx, "0.0.0".to_string());
    let links: &Signal<Vec<Link>> = create_signal(cx, Vec::new());
    let devices: &Signal<Vec<Device>> = create_signal(cx, Vec::new());
    let last_events: &Signal<Vec<Event>> = create_signal(cx, Vec::new());
    let last_actions: &Signal<Vec<Event>> = create_signal(cx, Vec::new());

    spawn_local_scoped(cx, async move {
        loop {
            fetch("/api/version", |response| {
                version.set(response);
            })
            .await;

            fetch("/api/devices", |response| {
                if let Ok(devs) = serde_json::from_str(&response) {
                    devices.set(devs);
                }
            })
            .await;

            fetch("/api/links", |response| {
                if let Ok(links_data) = serde_json::from_str(&response) {
                    links.set(links_data);
                }
            })
            .await;
            fetch("/api/last-events", |response| {
                let res:Result<Vec<Event>,Error>=serde_json::from_str(&response);
                if let Ok(events) = res {
                    let mut tmp_ev=Vec::new();
                    let mut tmp_ac=Vec::new();
                    for ev in events{
                        if ev.event != EventName::NewData {
                            if ev.event_type == EventType::Action {
                                tmp_ac.push(ev);
                            } else {
                                tmp_ev.push(ev);
                            }
                        }
                    }
                    last_events.set(tmp_ev);
                    last_actions.set(tmp_ac);
                }
            })
            .await;

            TimeoutFuture::new(1000).await;
        }
    });

    view! {cx,
            Router(
                integration=HistoryIntegration::new(),
                view=move |cx, route: &ReadSignal<AppRoutes>| {
                    view! {cx,
            ViewHead(version=version)
            ViewNav(links=links)
            RouterView(
                route=route,
                last_events=last_events,
                last_actions=last_actions,
                devices=devices
            )
            ViewFoot{}
        }
    }
    )
        }
}

fn main() {
    sycamore::render(|cx| {
        view! {cx,
            App{}
        }
    });
}
