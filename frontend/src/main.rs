//! The simplest fetch example.
use fireplace::{
    config::Link,
    devices::Device,
    eventhandler::{Event, TimedEvent},
};
use seed::prelude::*;
use serde_json;
mod components;
mod router;
mod utils;
mod views;

use components::{view_foot, view_head, view_nav};
use router::Page;

use crate::{router::route_view, utils::post};
use utils::fetch;

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
pub struct Model {
    pub version: Option<String>,
    pub devices: Vec<Device>,
    pub links: Vec<Link>,
    pub last_events: Vec<TimedEvent>,
    pub route: Option<Page>,
    pub last_actions: Vec<TimedEvent>,
}

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .stream(streams::interval(1000, || Msg::Fetch))
        .perform_cmd(async { Msg::Fetch });
    Model::default()
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    Fetch,
    SetView(Page),
    ReceivedVersion(String),
    ReceivedDevices(Vec<Device>),
    ReceivedLinks(Vec<Link>),
    ReceivedLastActions(Vec<TimedEvent>),
    ReceivedLastEvents(Vec<TimedEvent>),
    TriggerAction(Event),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Fetch => {
            orders
                .skip() // No need to rerender
                .perform_cmd(fetch("/api/version", |response| {
                    Some(Msg::ReceivedVersion(response))
                }))
                .perform_cmd(fetch("/api/devices", |response| {
                    if let Ok(devs) = serde_json::from_str(&response) {
                        Some(Msg::ReceivedDevices(devs))
                    } else {
                        None
                    }
                }))
                .perform_cmd(fetch("/api/links", |response| {
                    if let Ok(links) = serde_json::from_str(&response) {
                        Some(Msg::ReceivedLinks(links))
                    } else {
                        None
                    }
                }))
                .perform_cmd(fetch("/api/last_actions", |response| {
                    if let Ok(actions) = serde_json::from_str(&response) {
                        Some(Msg::ReceivedLastActions(actions))
                    } else {
                        None
                    }
                }))
                .perform_cmd(fetch("/api/last-events", |response| {
                    if let Ok(events) = serde_json::from_str(&response) {
                        Some(Msg::ReceivedLastEvents(events))
                    } else {
                        None
                    }
                }));
        }
        Msg::ReceivedVersion(user) => {
            model.version = Some(user);
        }
        Msg::ReceivedDevices(devs) => {
            model.devices = devs;
        }
        Msg::ReceivedLinks(links) => {
            model.links = links;
        }
        Msg::ReceivedLastActions(actions) => {
            model.last_actions = actions;
        }
        Msg::ReceivedLastEvents(events) => {
            model.last_events = events;
        }
        Msg::SetView(page) => {
            model.route = Some(page);
        }
        Msg::TriggerAction(event) => {
            orders
                .skip()
                .perform_cmd(post("/api/trigger-action", event, |_| None));
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        view_head(model),
        view_nav(model),
        route_view(model),
        view_foot(),
    ]
}

// ------ ------
//     Start
// ------ ------

fn main() {
    App::start("app", init, update, view);
}
