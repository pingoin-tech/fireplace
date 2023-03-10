use fireplace::{devices::Device, eventhandler::EventType};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DeviceProps {
    pub device: Device,
    pub on_click: Callback<EventType>,
}

#[function_component(DeviceField)]
pub fn device_field(DeviceProps { device, on_click }: &DeviceProps) -> Html {
    let name = if let Some(name) = &device.alias {
        name
    } else {
        &device.id
    };

    let actions: Html = device
        .available_actions
        .iter()
        .map(|action| {
            let onklick: Callback<MouseEvent> = {
                let on_click = on_click.clone();
                let event = action.clone();
                Callback::from(move |_ev: MouseEvent| on_click.emit(event.clone()))
            };

            html!(<button onclick={onklick}>{ action.action.clone() }</button>)
        })
        .collect();

    let values: Html = device
        .values
        .iter()
        .map(|v| html!(<><div>{v.0}</div><div>{v.1}</div></>))
        .collect();

    html! {
        <article class="dual-column" id={device.id.clone()}>
            <h3>{ name }</h3>
            <div>{"IP"}</div><div><a href={format!("http://{}",device.ip)} target={"_blank"}>{ device.ip.clone() }{"/"}</a><br/>{ device.mac.clone() }</div>
            <div>{"RSSI"}</div><div>{ device.rssi }</div>
            {values}
            <div>{"Actions"}</div><div>{actions}</div>
        </article>
    }
}
