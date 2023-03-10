use chrono::Utc;
use serde_json::Error;
use std::str::FromStr;

use crate::{
    devices::{get_device_from_list, insert_value_in_device, Device},
    eventhandler::EVENT_HANDLER,
    store::STORE,
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
                let mut dev = Device::default();
                let (shelly, actions, events) = device.to_shelly();
                dev.id = device.id.clone();
                dev.ip = device.ip.clone();
                dev.subdevice = DeviceType::Shelly(shelly);
                dev.values
                    .insert("firmware".to_string(), Value::String(device.fw_ver));
                dev.mac = device.mac;
                dev.last_message = Utc::now();
                dev.available_actions = actions;
                dev.available_events = events;

                STORE.open_locked(
                    |store| {
                        if let Some(config) = store.config.device_settings.get(&dev.id) {
                            dev.alias = Some(config.alias.clone());
                        }
                    },
                    (),
                );

                list.push(dev);
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
                                        create_val_key("relay", pos),
                                        Value::Bool(relay.ison),
                                    );
                                    if let Some(overpower) = relay.overpower {
                                        device.values.insert(
                                            create_val_key("relay-overpower", pos),
                                            Value::Bool(overpower),
                                        );
                                    }
                                }
                            }
                            if let Some(lights) = info_data.lights {
                                for (pos, light) in lights.iter().enumerate() {
                                    device.values.insert(
                                        create_val_key("light-on", pos),
                                        Value::Bool(light.ison),
                                    );
                                    device.values.insert(
                                        create_val_key("brightness", pos),
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
                        create_val_key((subdev.to_string() + "-power").as_str(), index),
                        Value::Number(val),
                    );
                }
            }
            Some("energy") => {
                if let Ok(val) = f32::from_str(telegram.payload.as_str()) {
                    insert_value_in_device(
                        telegram.id.clone(),
                        create_val_key((subdev.to_string() + "-energy").as_str(), index),
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
                    create_val_key((subdev.to_string() + "-on").as_str(), index),
                    Value::Bool(on),
                );
            }
        }
    }
    trigger_new_data(telegram.id)
}

fn trigger_new_data(id: String) {
    EVENT_HANDLER.open_locked(
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
                    create_val_key("roller-status", index),
                    Value::String(telegram.payload),
                );
            }
            Some("pos") => {
                if let Ok(val) = f32::from_str(telegram.payload.as_str()) {
                    insert_value_in_device(
                        telegram.id,
                        create_val_key("roller-position", index),
                        Value::Number(val),
                    );
                }
            }
            Some("energy") => {
                if let Ok(val) = f32::from_str(telegram.payload.as_str()) {
                    insert_value_in_device(
                        telegram.id,
                        create_val_key("roller-energy", index),
                        Value::Number(val),
                    );
                }
            }
            Some("power") => {
                if let Ok(val) = f32::from_str(telegram.payload.as_str()) {
                    insert_value_in_device(
                        telegram.id,
                        create_val_key("roller-power", index),
                        Value::Number(val),
                    );
                }
            }
            Some("stop_reason") => {
                insert_value_in_device(
                    telegram.id,
                    create_val_key("roller-stop-reason", index),
                    Value::String(telegram.payload),
                );
            }
            _ => {}
        }
        trigger_new_data(id);
    }
}

fn create_val_key(name: &str, pos: usize) -> String {
    if pos == 0 {
        name.to_string()
    } else {
        format!("{}-{}", name, pos)
    }
}
