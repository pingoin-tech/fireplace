use std::collections::HashMap;

use chrono::Utc;
use serde_json::Error;
use std::str::FromStr;

use crate::{
    devices::{get_device_from_list, Device, DeviceType},
    eventhandler::{get_event_handler, Value},
};

use super::{
    incoming_data::{ShellyAnnounce, ShellyInfo},
    Shelly, Telegram,
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
                let (shelly, actions, events) = Shelly::from_announce(device);
                let sub_device = DeviceType::Shelly(shelly);
                list.push(Device {
                    id: id,
                    ip: ip,
                    last_message: Utc::now(),
                    subdevice: sub_device,
                    available_actions: actions,
                    available_events: events,
                    rssi: 0,
                    values: HashMap::new(),
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
                        DeviceType::Shelly(shel) => {
                            shel.inputs = info_data.inputs;
                            shel.meters = info_data.meters;
                            shel.relays = info_data.relays;
                            shel.update = info_data.update;
                            shel.lights = info_data.lights;
                            shel.rollers = info_data.rollers;
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
                "State input: {}/{}: {:?}",
                dev.id, telegram.topic, telegram.payload
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

pub fn decode_voltage(telegram: Telegram) {
    if let Ok(val) = f32::from_str(telegram.payload.as_str()) {
        get_device_from_list(
            telegram.id,
            |shelly| {
                shelly
                    .values
                    .insert("voltage".to_string(), Value::Number(val));
            },
            |_| {},
            (),
        )
    }
}

pub fn decode_subdevice(telegram: Telegram, subdev: &str) {
    if let Some(index) = telegram.subdevice_number {
        match telegram.subdevice.as_deref() {
            Some("power") => {
                if let Ok(val) = f32::from_str(telegram.payload.as_str()) {
                    get_device_from_list(
                        telegram.id.clone(),
                        |shelly| {
                            shelly
                                .values
                                .insert(format!("{}/{}/power", subdev, index), Value::Number(val));
                        },
                        |_| {},
                        (),
                    )
                }
            }
            Some("energy") => {
                if let Ok(val) = f32::from_str(telegram.payload.as_str()) {
                    get_device_from_list(
                        telegram.id.clone(),
                        |shelly| {
                            shelly
                                .values
                                .insert(format!("{}/{}/energy", subdev, index), Value::Number(val));
                        },
                        |_| {},
                        (),
                    )
                }
            }
            Some(_) => {}
            None => {
                let mut on = false;
                if telegram.payload.as_str() == "on" {
                    on = true;
                }
                get_device_from_list(
                    telegram.id.clone(),
                    |shelly| {
                        shelly
                            .values
                            .insert(format!("{}/{}/on", subdev, index), Value::Bool(on));
                    },
                    |_| {},
                    (),
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
