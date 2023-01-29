use super::devices::{self, Device, DeviceType};
use chrono::Utc;
use rumqttc::Publish;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::str::Split;

#[derive(Serialize, Deserialize, Debug)]
pub struct ShellyAnnounce {
    pub id: String,
    pub model: String,
    pub mac: String,
    pub ip: String,
    pub new_fw: bool,
    pub fw_ver: String,
    pub mode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Shelly {
    pub fw_ver: String,
    pub shelly_type: ShellyType,
}

#[derive(Serialize, Deserialize, Debug)]

pub enum ShellyType {
    Shelly1,
    ShellyDimmer,
    Shelly25Roller,
    Shelly25Switch,
}

impl Shelly {
    fn from_announce(data: ShellyAnnounce) -> Shelly {
        let mut shelly_type = ShellyType::Shelly1;
        match data.model.as_str() {
            "SHSW-25" => {
                if data.mode == Some(String::from("roller")) {
                    shelly_type = ShellyType::Shelly25Roller;
                } else {
                    shelly_type = ShellyType::Shelly25Switch;
                }
            }
            "SHSW-1" => {
                shelly_type = ShellyType::Shelly1;
            }
            "SHDM-2" => {
                shelly_type = ShellyType::ShellyDimmer;
            }
            _ => {}
        }

        Shelly {
            fw_ver: data.fw_ver,
            shelly_type: shelly_type,
        }
    }
}

pub fn decode_shelly_sub(content: &Publish, mut path: Split<&str>) {
    match path.next() {
        Some("announce") => {
            decode_announce(content);
        }
        Some("command") => {}
        Some(id) => match path.next() {
            Some("announce") => {
                decode_announce(content);
            }
            _ => {
                devices::get_device_from_list(
                    String::from(id),
                    |dev| {
                        dev.last_message = Utc::now();
                        println!("State input: {}", dev.id);
                    },
                    |_| println!("Unknown device: {}", id),
                );
            }
        },
        _ => {}
    }
}

fn decode_announce(content: &Publish) {
    let dev_res: Result<ShellyAnnounce, Error> = serde_json::from_slice(&content.payload);
    match dev_res {
        Ok(device) => devices::get_device_from_list(
            device.id.clone(),
            |dev| {
                dev.last_message = Utc::now();
                println!("{} already exists", dev.id);
            },
            |list| {
                let id = device.id.clone();
                let ip = device.ip.clone();
                let sub_device = DeviceType::Shelly(Shelly::from_announce(device));
                list.push(Device {
                    id: id,
                    ip: ip,
                    last_message: Utc::now(),
                    subdevice: sub_device,
                });
            },
        ),
        Err(err) => println!("{:?}", err),
    }
}
