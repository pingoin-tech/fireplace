use std::collections::BTreeMap;

use chrono::Utc;
use serde_json::Error;
use std::str::FromStr;

use crate::{
    devices::{get_device_from_list, insert_value_in_device, Device},
    eventhandler::get_event_handler,
};
use fireplace::devices::DeviceType;
use fireplace::eventhandler::Value;

use super::{
    incoming_data::{ShellyAnnounce, ShellyInfo},
    Telegram,
};

pub fn decode_announce(content: Telegram) {
    let dev_res: Result<ShellyAnnounce, Error> = serde_json::from_str(&content.payload.as_str());
    match dev_res {
        Ok(device) => get_device_from_list(
            device.id.clone(),
            |dev| {
                dev.last_message = Utc::now();
            },
            |list| {
                let id = device.id.clone();
                let ip = device.ip.clone();
                let (shelly, actions, events) = device.to_shelly();
                let sub_device = DeviceType::Shelly(shelly);
                let mut values=BTreeMap::new();
                values.insert("firmware".to_string(), Value::String(device.fw_ver));
                list.push(Device {
                    id: id,
                    ip: ip,
                    mac: device.mac,
                    last_message: Utc::now(),
                    subdevice: sub_device,
                    available_actions: actions,
                    available_events: events,
                    rssi: 0,
                    values: values,
                });
            },
            (),
        ),
        Err(err) => println!("{:?}", err),
    }
}

pub fn decode_info(telegram: Telegram) {
    let dev_res: Result<ShellyInfo, Error> = serde_json::from_str(&telegram.payload.as_str());
    match dev_res {
        Ok(info_data) => {
            get_device_from_list(
                telegram.id.clone(),
                |device| {
                    device.rssi = info_data.wifi_sta.rssi;
                    match &mut device.subdevice {
                        DeviceType::Shelly(_shel) => {
                            if let Some(relays) = info_data.relays {
                                for (pos, relay) in relays.iter().enumerate() {
                                    device.values.insert(
                                        format!("{}/{}/on", "relay", pos),
                                        Value::Bool(relay.ison),
                                    );
                                    if let Some(overpower) = relay.overpower {
                                        device.values.insert(
                                            format!("{}/{}/overpower", "relay", pos),
                                            Value::Bool(overpower),
                                        );
                                    }
                                }
                            }
                            if let Some(lights) = info_data.lights {
                                for (pos, light) in lights.iter().enumerate() {
                                    device.values.insert(
                                        format!("{}/{}/on", "light", pos),
                                        Value::Bool(light.ison),
                                    );
                                    device.values.insert(
                                        format!("{}/{}/brightness", "light", pos),
                                        Value::Number(light.brightness as f32),
                                    );
                                }
                            }

                            device.values.insert(
                                "uptime".to_string(),
                                Value::Number(info_data.uptime as f32),
                            );
                            if let Some(op) = info_data.overpower {
                                device
                                    .values
                                    .insert("overpower".to_string(), Value::Bool(op));
                            }
                            if let Some(ot) = info_data.overtemperature {
                                device
                                    .values
                                    .insert("overtemperature".to_string(), Value::Bool(ot));
                            }
                        }
                        _ => {}
                    };
                },
                |_| (),
                (),
            );
        }
        Err(err) => println!("{:?}", err),
    }
}

pub fn decode_other(telegram: Telegram) {
    get_device_from_list(
        telegram.id.clone(),
        |dev| {
            dev.last_message = Utc::now();
            println!(
                "State input: {}/{:?}/{:?}/{}: {:?}",
                dev.id,
                telegram.subdevice,
                telegram.subdevice_number,
                telegram.topic,
                telegram.payload
            );
        },
        |_| {
            println!(
                "Unknown device: {}/{}: {:?}",
                telegram.id, telegram.topic, telegram.payload
            )
        },
        (),
    );
}

pub fn decode_value(telegram: Telegram, value: &str) {
    if let Ok(val) = f32::from_str(telegram.payload.as_str()) {
        insert_value_in_device(telegram.id, value.to_string(), Value::Number(val));
    }
}

pub fn decode_subdevice(telegram: Telegram, subdev: &str) {
    if let Some(index) = telegram.subdevice_number {
        match telegram.subdevice.as_deref() {
            Some("power") => {
                if let Ok(val) = f32::from_str(telegram.payload.as_str()) {
                    insert_value_in_device(
                        telegram.id.clone(),
                        format!("{}/{}/power", subdev, index),
                        Value::Number(val),
                    );
                }
            }
            Some("energy") => {
                if let Ok(val) = f32::from_str(telegram.payload.as_str()) {
                    insert_value_in_device(
                        telegram.id.clone(),
                        format!("{}/{}/energy", subdev, index),
                        Value::Number(val),
                    );
                }
            }
            Some(_) => {}
            None => {
                let mut on = false;
                if telegram.payload.as_str() == "on" {
                    on = true;
                }
                insert_value_in_device(
                    telegram.id.clone(),
                    format!("{}/{}/on", subdev, index),
                    Value::Bool(on),
                );
            }
        }
    }
    trigger_new_data(telegram.id)
}

fn trigger_new_data(id: String) {
    get_event_handler(
        |handler| handler.trigger_event(format!("{}/new_data", id)),
        (),
    )
}

pub fn decode_roller(telegram: Telegram) {
    if let Some(index) = telegram.subdevice_number {
        let id = telegram.id.clone();
        match telegram.subdevice.as_deref() {
            None => {
                insert_value_in_device(
                    telegram.id,
                    format!("roller/{}/status", index),
                    Value::String(telegram.payload),
                );
            }
            Some("pos") => {
                if let Ok(val) = f32::from_str(telegram.payload.as_str()) {
                    insert_value_in_device(
                        telegram.id,
                        format!("roller/{}/position", index),
                        Value::Number(val),
                    );
                }
            }
            Some("energy") => {
                if let Ok(val) = f32::from_str(telegram.payload.as_str()) {
                    insert_value_in_device(
                        telegram.id,
                        format!("roller/{}/energy", index),
                        Value::Number(val),
                    );
                }
            }
            Some("power") => {
                if let Ok(val) = f32::from_str(telegram.payload.as_str()) {
                    insert_value_in_device(
                        telegram.id,
                        format!("roller/{}/power", index),
                        Value::Number(val),
                    );
                }
            }
            Some("stop_reason") => {
                insert_value_in_device(
                    telegram.id,
                    format!("roller/{}/stop_reason", index),
                    Value::String(telegram.payload),
                );
            }
            _ => {}
        }
        trigger_new_data(id);
    }
}