use fireplace::{devices::Device, eventhandler::EventType};
use seed::{prelude::*, *};

use crate::Msg;

pub fn device_field(device: &Device) -> Node<Msg> {
    let name = if let Some(name) = &device.alias {
        name
    } else {
        &device.id
    };

    let mut actions: Vec<Node<Msg>> = Vec::new();

    for action in &device.available_actions {
        let event = EventType {
            id: action.id.clone(),
            action: action.action.clone(),
            value: action.value.clone(),
            subdevice: action.subdevice.clone(),
        };
        actions.push(button!(
            ev(Ev::Click, |_| Msg::TriggerAction(event)),
            &action.action
        ));
    }

    /*
        device
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

    */
    let mut values: Vec<Node<Msg>> = Vec::new();

    for value in &device.values {
        values.push(div!(value.0));
        values.push(div!(format!("{}", value.1)));
    }
    article![
        C!("dual-column"),
        h3!(name),
        div!("IP"),
        div!(
            a!(
                attrs!(At::Href=>format!("http://{}",device.ip),At::Target=>"_blank"),
                &device.ip
            ),
            br!(),
            &device.mac
        ),
        div!("RSSI"),
        div!(format!("{}", device.rssi)),
        values,
        div!("actions"),
        div![actions]
    ]
}
