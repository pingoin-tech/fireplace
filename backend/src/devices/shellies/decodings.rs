use chrono::{DateTime, Duration, Utc};
use serde_json::Error;
use std::str::FromStr;

use super::{
    incoming_data::{InputEvent, ShellyAnnounce, ShellyInfo},
    Telegram,
};
use crate::{
    devices::{get_device_from_list, insert_value_in_device, Device},
    eventhandler::EVENT_HANDLER,
    store::STORE,
    utils::format_mac,
};
use fireplace::eventhandler::{EventName, EventType, Value};
use fireplace::{devices::DeviceType, eventhandler::Event};

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
                dev.mac = format_mac(device.mac);
                dev.last_message = Utc::now();
                dev.last_data = Utc::now();
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

                            let seconds = info_data.uptime % 60;
                            let minutes = (info_data.uptime / 60) % 60;
                            let hours = (info_data.uptime / 60) / 60 % 24;
                            let days = (info_data.uptime / 60) / 60 / 24;

                            device.values.insert(
                                "uptime".to_string(),
                                Value::String(format!(
                                    "{}d{}h{}min{}s",
                                    days, hours, minutes, seconds
                                )),
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
    let mut old_data_time = (false, Utc::now());
    if let Some(index) = telegram.subdevice_number {
        match telegram.subdevice.as_deref() {
            Some("power") => {
                if let Ok(val) = f32::from_str(telegram.payload.as_str()) {
                    old_data_time = insert_value_in_device(
                        telegram.id.clone(),
                        create_val_key((subdev.to_string() + "-power").as_str(), index),
                        Value::Number(val),
                    );
                }
            }
            Some("energy") => {
                if let Ok(mut val) = f32::from_str(telegram.payload.as_str()) {
                    val = val / (60000.0); // from Watt*Minute to kWh
                    old_data_time = insert_value_in_device(
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
                old_data_time = insert_value_in_device(
                    telegram.id.clone(),
                    create_val_key((subdev.to_string() + "-on").as_str(), index),
                    Value::Bool(on),
                );
            }
        }
    }
    trigger_new_data(telegram.id, old_data_time.1)
}

fn trigger_new_data(id: String, old_time: DateTime<Utc>) {
    let event = Event::new_event(&id, EventName::NewData);

    let diff = Utc::now() - old_time;
    if diff > Duration::seconds(1) {
        EVENT_HANDLER.open_locked(|handler| handler.trigger_event(event), ())
    }
}

pub fn decode_roller(telegram: Telegram) {
    let mut old_data_time = (false, Utc::now());
    if let Some(index) = telegram.subdevice_number {
        let id = telegram.id.clone();
        match telegram.subdevice.as_deref() {
            None => {
                old_data_time = update_roller_stat(
                    telegram.id,
                    create_val_key("roller-status", index),
                    Value::String(telegram.payload),
                );
            }
            Some("pos") => {
                if let Ok(val) = f32::from_str(telegram.payload.as_str()) {
                    old_data_time = insert_value_in_device(
                        telegram.id,
                        create_val_key("roller-position", index),
                        Value::Number(val),
                    );
                }
            }
            Some("energy") => {
                if let Ok(mut val) = f32::from_str(telegram.payload.as_str()) {
                    val = val / (60000.0); // from Watt*Minute to kWh
                    old_data_time = insert_value_in_device(
                        telegram.id,
                        create_val_key("roller-energy", index),
                        Value::Number(val),
                    );
                }
            }
            Some("power") => {
                if let Ok(val) = f32::from_str(telegram.payload.as_str()) {
                    old_data_time = insert_value_in_device(
                        telegram.id,
                        create_val_key("roller-power", index),
                        Value::Number(val),
                    );
                }
            }
            Some("stop_reason") => {
                old_data_time = insert_value_in_device(
                    telegram.id,
                    create_val_key("roller-stop-reason", index),
                    Value::String(telegram.payload),
                );
            }
            _ => {}
        }
        trigger_new_data(id, old_data_time.1);
    }
}

fn create_val_key(name: &str, pos: usize) -> String {
    if pos == 0 {
        name.to_string()
    } else {
        format!("{}-{}", name, pos)
    }
}

fn update_roller_stat(id: String, key: String, val: Value) -> (bool, DateTime<Utc>) {
    get_device_from_list(
        id,
        |device| {
            let old_time = device.last_data.clone();
            device.last_data = Utc::now();
            if val.clone() != Value::String("stop".to_string()) {
                device
                    .values
                    .insert(key.replace("status", "last-direction"), val.clone());
            }
            device.values.insert(key, val);
            (true, old_time)
        },
        |_| (false, Utc::now()),
        (false, Utc::now()),
    )
}

pub fn decode_input_event(telegram: Telegram) {
    let input_event: Result<InputEvent, Error> = serde_json::from_str(&telegram.payload.as_str());

    match input_event {
        Ok(input_event) => {
            let event_name = match input_event.event {
                'L' => EventName::InputLong,
                _ => EventName::InputShort,
            };

            let event = Event {
                id: telegram.id,
                event: event_name,
                timestamp: Utc::now(),
                event_type: EventType::Event,
                handled: false,
                subdevice: telegram.subdevice,
                index: None,
            };
            EVENT_HANDLER.open_locked(|handler| handler.trigger_event(event), ());
        }
        Err(err) => println!("{:?}", err),
    }
}
