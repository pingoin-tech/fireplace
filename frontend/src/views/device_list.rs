use fireplace::{devices::Device, eventhandler::EventType};
use yew::prelude::*;

use crate::components::DeviceField;

#[derive(Properties, PartialEq)]
pub struct DeviceListProps {
    pub devices: Vec<Device>,
    pub on_click: Callback<EventType>,
}

#[function_component(DeviceList)]
pub fn device_list(DeviceListProps { devices, on_click }: &DeviceListProps) -> Html {
    let dev: Html = devices
        .iter()
        .map(|device| html!(<DeviceField device={device.clone()} on_click={on_click}/>))
        .collect();
    html!(
        <main class="tripple-column">
        <h2>{"all devices"}</h2>
        {dev}
        </main>
    )
}
