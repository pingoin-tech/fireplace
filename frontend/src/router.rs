use seed::{prelude::*, *};

use crate::views::{device_list, events};
use crate::{Model, Msg};

pub enum Page {
    DeviceList,
    LastAction,
}

pub fn route_view(model: &Model) -> Node<Msg> {
    if let Some(route) = &model.route {
        match route {
            Page::DeviceList => device_list(model),
            Page::LastAction => events(model),
        }
    } else {
        device_list(model)
    }
}
