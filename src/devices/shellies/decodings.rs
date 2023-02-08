use std::{collections::HashMap, str, str::Split};

use chrono::Utc;
use rumqttc::Publish;
use serde_json::Error;
use std::str::FromStr;

use crate::{
    devices::{get_device_from_list, Device, DeviceType},
    eventhandler::{get_event_handler, Value},
};

use super::{
    incoming_data::{ShellyAnnounce, ShellyInfo},
    Shelly,
};

pub fn decode_announce(content: &Publish) {
    let dev_res: Result<ShellyAnnounce, Error> = serde_json::from_slice(&content.payload);
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

pub fn decode_info(content: &Publish, id: String) {
    let dev_res: Result<ShellyInfo, Error> = serde_json::from_slice(&content.payload);
    match dev_res {
        Ok(info_data) => {
            get_device_from_list(
                id.clone(),
                |device| {
                    device.rssi = info_data.wifi_sta.rssi;
                    match &mut device.subdevice {
                        DeviceType::Shelly(shel) => {
                            shel.inputs = info_data.inputs;
                            shel.meters = info_data.meters;
                            shel.relays = info_data.relays;
                            shel.update = info_data.update;
                            shel.uptime = info_data.uptime;
                            shel.lights = info_data.lights;
                            shel.rollers = info_data.rollers;
                            shel.overpower = info_data.overpower;
                            shel.overtemperature = info_data.overtemperature;
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

pub fn decode_other(path: &str, id: String, content: &Publish) {
    get_device_from_list(
        id.clone(),
        |dev| {
            dev.last_message = Utc::now();
            println!("State input: {}/{}: {:?}", dev.id, path, content.payload);
        },
        |_| println!("Unknown device: {}/{}: {:?}", id, path, content.payload),
        (),
    );
}

pub fn decode_voltage(content: &Publish, id: String) {
    if let Ok(string) = str::from_utf8(&content.payload) {
        if let Ok(val) = f32::from_str(string) {
            get_device_from_list(
                id,
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
}

pub fn decode_relay(content: &Publish, id: String, mut path: Split<&str>) {
    if let Some(index) = path.next() {
        let payload_result = str::from_utf8(&content.payload);
        if let Ok(status) = payload_result {
            get_device_from_list(
                id.clone(),
                |shelly| {
                    shelly.values.insert(
                        format!("relay/{}", index),
                        Value::String(status.to_string()),
                    );
                },
                |_| {},
                (),
            );
        };
    };
    trigger_new_data(id)
}

fn trigger_new_data(id: String) {
    get_event_handler(
        |handler| handler.trigger_event(format!("{}/new_data", id)),
        (),
    )
}

pub fn decode_light(content: &Publish, id: String, mut path: Split<&str>) {
    let index_op: Option<usize> = match path.next() {
        None => None,
        Some("0") => Some(0),
        Some("1") => Some(1),
        _ => None,
    };
    match path.next() {
        Some("power") => {
            if let Ok(string) = str::from_utf8(&content.payload) {
                if let Ok(val) = f32::from_str(string) {
                    get_device_from_list(
                        id.clone(),
                        |shelly| {
                            shelly
                                .values
                                .insert("power".to_string(), Value::Number(val));
                        },
                        |_| {},
                        (),
                    )
                }
            }
        }
        Some("energy") => {
            if let Ok(string) = str::from_utf8(&content.payload) {
                if let Ok(val) = f32::from_str(string) {
                    get_device_from_list(
                        id.clone(),
                        |shelly| {
                            shelly
                                .values
                                .insert("power".to_string(), Value::Number(val));
                        },
                        |_| {},
                        (),
                    )
                }
            }
        }
        Some(_) => {}
        None => {
            if let Some(index) = index_op {
                let payload_result = str::from_utf8(&content.payload);
                if let Ok(status) = payload_result {
                    get_device_from_list(
                        id.clone(),
                        |shelly| {
                            shelly.values.insert(
                                format!("light/{}", index),
                                Value::String(status.to_string()),
                            );
                        },
                        |_| {},
                        (),
                    );
                };
            }
        }
    }
    trigger_new_data(id)
}
