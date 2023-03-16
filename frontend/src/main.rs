//! The simplest fetch example.
use fireplace::{config::Link, devices::Device, eventhandler::EventType};
use seed::prelude::*;
use serde_json;
mod components;
mod utils;
mod views;

use components::{view_foot, view_head, view_nav};

use crate::{utils::post, views::device_list};
use utils::fetch;

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
pub struct Model {
    pub version: Option<String>,
    pub devices: Vec<Device>,
    pub links: Vec<Link>,
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
    ReceivedVersion(String),
    ReceivedDevices(Vec<Device>),
    ReceivedLinks(Vec<Link>),
    TriggerAction(EventType),
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
        device_list(model),
        view_foot(),
    ]
}

// ------ ------
//     Start
// ------ ------

fn main() {
    App::start("app", init, update, view);
}
