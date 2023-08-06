use fireplace::{devices::Device, eventhandler::Event};
use sycamore::prelude::*;
use sycamore_router::Route;

use crate::views::{DeviceList, EventView};
//use crate::{Model, Msg};

#[derive(Route)]
pub enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/device-list")]
    DeviceList,
    #[to("/last-events")]
    LastEvents,
    #[not_found]
    NotFound,
}

#[component(inline_props)]
pub fn RouterView<'a, G: Html>(
    cx: Scope<'a>,
    route: &'a ReadSignal<AppRoutes>,
    last_events: &'a ReadSignal<Vec<Event>>,
    last_actions: &'a ReadSignal<Vec<Event>>,
    devices: &'a ReadSignal<Vec<Device>>,
) -> View<G> {
    view! { cx,
        (match route.get().as_ref() {
            AppRoutes::Index => view! { cx,
                DeviceList(devices=devices)
            },
            AppRoutes::DeviceList => view! { cx,
                DeviceList(devices=devices)
            },
            AppRoutes::LastEvents => view! { cx,
                EventView(last_events=last_events,last_actions=last_actions)
            },
            AppRoutes::NotFound => view! { cx,
               article{h2{"404 Not Found"}}
            },
        })
    }
}
