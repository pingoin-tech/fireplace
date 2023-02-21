use fireplace::{devices::Device, eventhandler::EventType};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DeviceListProps {
    pub devices: Vec<Device>,
    pub on_click: Callback<EventType>,
}

#[function_component(DeviceList)]
pub fn device_list(DeviceListProps { devices, on_click }: &DeviceListProps) -> Html {
    devices
        .iter()
        .map(|device| {
            let values:Html= device.values.iter().map(|v|{
                html!(<li>{format!("{}: {}",v.0,v.1)}</li>)
            }).collect();

            let actions:Html= device.available_actions.iter().map(|action|{
                let onklick:Callback<MouseEvent>={
                    let on_click = on_click.clone();
                        let event =EventType{
                            id:device.id.clone(),
                            action:action.clone(),
                            subdevice:None,
                            value:None,
                        };
                        Callback::from(move |_ev:MouseEvent| {
                            on_click.emit(event.clone())
                        })
                };

                html!(<button onclick={onklick}>{ action }</button>)
            }).collect();

            html! {
                <tr key={device.id.clone()}>
                    <td>{ device.id.clone() }</td>
                    <td><a href={format!("http://{}",device.ip)} target={"_blank"}>{ device.ip.clone() }</a></td>
                    <td>{ device.rssi }</td>
                    <td><ul>{values}</ul></td>
                    <td>{actions}</td>
                </tr>
            }
        })
        .collect()
}
