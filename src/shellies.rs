use super::devices::DeviceType;
use crate::devices::Device;
use chrono::Utc;
use rumqttc::Publish;
use serde::{Deserialize, Serialize};
use serde_json::Error;
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
pub struct Shelly1 {
    pub id: String,
    pub ip: String,
    pub fw_ver: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Shelly25Roller {
    pub id: String,
    pub ip: String,
    pub fw_ver: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShellyDimmer {
    pub id: String,
    pub ip: String,
    pub fw_ver: String,
}

impl Shelly1 {
    fn from_announce(data: ShellyAnnounce) -> Shelly1 {
        Shelly1 {
            id: data.id,
            ip: data.ip,
            fw_ver: data.fw_ver,
        }
    }
}
impl Shelly25Roller {
    fn from_announce(data: ShellyAnnounce) -> Shelly25Roller {
        Shelly25Roller {
            id: data.id,
            ip: data.ip,
            fw_ver: data.fw_ver,
        }
    }
}

impl ShellyDimmer {
    fn from_announce(data: ShellyAnnounce) -> ShellyDimmer {
        ShellyDimmer {
            id: data.id,
            ip: data.ip,
            fw_ver: data.fw_ver,
        }
    }
}

pub fn decode_shelly_sub(content: &Publish, mut path: Split<&str>) {
    match path.next() {
        Some("announce") => {
            let dev_res: Result<ShellyAnnounce, Error> = serde_json::from_slice(&content.payload);
            match dev_res {
                Ok(device) => match SENSOR_LIST.lock() {
                    Ok(mut list_option) => {
                        if let Some(list) = list_option.as_mut() {
                            match list.into_iter().find(|x| x.id == device.id) {
                                Some(dev) => {
                                    dev.last_message = Utc::now();
                                    println!(
                                        "{} already exists\nUpdate not jet implemented",
                                        dev.id
                                    );
                                }
                                None => {
                                    let mut sub_device: DeviceType = DeviceType::Empty;
                                    let id = device.id.clone();
                                    match device.model.as_str() {
                                        "SHSW-25" => {
                                            if device.mode == Some(String::from("roller")) {
                                                sub_device = DeviceType::Shelly25RollerType(
                                                    Shelly25Roller::from_announce(device),
                                                );
                                            }
                                        }
                                        "SHSW-1" => {
                                            sub_device = DeviceType::Shelly1Type(
                                                Shelly1::from_announce(device),
                                            )
                                        }
                                        "SHDM-2" => {
                                            sub_device = DeviceType::ShellyDimmerType(
                                                ShellyDimmer::from_announce(device),
                                            )
                                        }
                                        _ => {}
                                    }

                                    list.push(Device {
                                        id: id,
                                        last_message: Utc::now(),
                                        subdevice: sub_device,
                                    });
                                }
                            }
                        }
                    }
                    Err(err) => println!("{:?}", err),
                },
                Err(err) => println!("{:?}", err),
            }
        }
        _ => {}
    }
}
