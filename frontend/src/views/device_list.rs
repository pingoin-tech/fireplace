use seed::{prelude::*, *};

use crate::{components::device_field, Model, Msg};

pub fn device_list(model: &Model) -> Node<Msg> {
    let mut devices: Vec<Node<Msg>> = Vec::new();

    for device in &model.devices {
        devices.push(device_field(device));
    }

    main![C!["tripple-column"], h2!("all devices"), devices]
}
