use crate::devices::{Device, DeviceType};

use self::{incoming_data::{
    InputStat, LightStat, MeterStat, RelaysState, RollerStat, UpdateStat, WifiState,
}, decodings::{decode_other, decode_relay}};

use super::devices;
use chrono::Utc;
use rumqttc::Publish;
use serde::{Deserialize, Serialize};
use std::str::Split;

mod incoming_data;
mod decodings;
use incoming_data::{ShellyAnnounce};
use decodings::{decode_announce,decode_info};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shelly {
    pub fw_ver: String,
    pub shelly_type: ShellyType,
    pub wifi_sta: WifiState,
    pub relays: Option<Vec<RelaysState>>,
    pub lights: Option<Vec<LightStat>>,
    pub rollers: Option<Vec<RollerStat>>,
    pub update: UpdateStat,
    pub meters: Vec<MeterStat>,
    pub inputs: Vec<InputStat>,
    pub overtemperature: Option<bool>,
    pub overpower: Option<bool>,
    pub uptime: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]

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
            wifi_sta: WifiState::default(),
            relays: None,
            update: UpdateStat::default(),
            meters: Vec::new(),
            inputs: Vec::new(),
            uptime: 0,
            lights: None,
            rollers: None,
            overtemperature: None,
            overpower: None,
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
            Some("command") => {}
            Some("online") => {}
            Some("relay")=> decode_relay(content,String::from(id), path),
            Some("info") => decode_info(content, String::from(id)),
            Some(path) => decode_other(path, String::from(id)),
            None => {}
        },
        _ => {}
    }
}


pub fn open_shelly_fom_list<Fs, Ff>(id: String, found: Fs, not_found: Ff)
where
    Fs: FnOnce(&mut Shelly),
    Ff: FnOnce(&mut Vec<Device>),
{
    devices::get_device_from_list(
        id,
        |device| {
            device.last_message = Utc::now();
            let sub_dev = &mut device.subdevice;
            match sub_dev {
                DeviceType::Shelly(shel) => {
                    found(shel)
                }
                _ => {}
            }
        },
        not_found,
    );
}