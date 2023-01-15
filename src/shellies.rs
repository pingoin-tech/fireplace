use super::devices::DeviceType::ShellyType;
use crate::devices::Device;
use chrono::Utc;
use rumqttc::Publish;
use serde::{Deserialize, Serialize};
use std::str::Split;

use super::devices::SENSOR_LIST;

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
    pub id: String,
    pub model: String,
    pub ip: String,
    pub fw_ver: String,
}

impl Shelly {
    fn from_announce(data: ShellyAnnounce) -> Shelly {
        Shelly {
            id: data.id,
            model: data.model,
            ip: data.ip,
            fw_ver: data.fw_ver,
        }
    }
}

pub fn decode_shelly_sub(content: &Publish, mut path: Split<&str>) {
    match path.next() {
        Some("announce") => match serde_json::from_slice(&content.payload) {
            Ok(device) => {
                let shelly = Shelly::from_announce(device);
                match SENSOR_LIST.lock() {
                    Ok(mut list_option) => {
                        if let Some(list) = list_option.as_mut() {
                            if list.contains_key(&shelly.id) {
                                println!("{} already exists", shelly.id);
                            } else {
                                list.insert(
                                    shelly.id.clone(),
                                    Device {
                                        last_message: Utc::now(),
                                        subdevice: ShellyType(shelly),
                                    },
                                );
                                println!("{:?}", &list);
                            }
                        }
                    }
                    Err(err) => println!("{:?}", err),
                }
            }
            Err(err) => println!("{:?}", err),
        },
        _ => {}
    }
}
