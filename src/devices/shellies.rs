use self::{
    decodings::{decode_other, decode_relay, decode_voltage},
    incoming_data::{
        InputStat, LightStat, MeterStat, RelaysState, RollerStat, UpdateStat, WifiState,
    },
};
use crate::{
    devices::{Device, DeviceType},
    eventhandler::ActionType,
};
use ts_rs::TS;

use super::get_device_from_list;
use chrono::Utc;
use rumqttc::Publish;
use serde::{Deserialize, Serialize};
use std::str::Split;

mod decodings;
mod incoming_data;
use decodings::{decode_announce, decode_info};
use incoming_data::ShellyAnnounce;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct Shelly {
    pub fw_ver: String,
    pub shelly_type: ShellyType,
    pub wifi_sta: WifiState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relays: Option<Vec<RelaysState>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lights: Option<Vec<LightStat>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollers: Option<Vec<RollerStat>>,
    pub update: UpdateStat,
    pub meters: Vec<MeterStat>,
    pub inputs: Vec<InputStat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overtemperature: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overpower: Option<bool>,
    pub uptime: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, TS)]
#[ts(export)]
pub enum ShellyType {
    Shelly1,
    ShellyDimmer,
    Shelly25Roller,
    Shelly25Switch,
}

impl Shelly {
    pub fn trigger_action(&mut self, action_path: String, id: String) -> ActionType {
        let base_path = format!("shellies/{}/", id);
        let mut splitted = action_path.split("/");

        match splitted.next() {
            Some("announce") => {
                ActionType::MqttAction(format!("{}command", base_path), String::from("announce"))
            }
            _ => ActionType::NotAvailable,
        }
    }

    fn from_announce(data: ShellyAnnounce) -> (Shelly, Vec<String>, Vec<String>) {
        let mut shelly_type = ShellyType::Shelly1;

        let actions = vec!["announce".to_string()];
        let events = vec!["new_data".to_string()];
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

        (
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
                voltage: None,
            },
            actions,
            events,
        )
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
            Some("relay") => decode_relay(content, String::from(id), path),
            Some("info") => decode_info(content, String::from(id)),
            Some("voltage") => decode_voltage(content, String::from(id)),
            Some(path) => decode_other(path, String::from(id), content),
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
    get_device_from_list(
        id,
        |device| {
            device.last_message = Utc::now();
            let sub_dev = &mut device.subdevice;
            match sub_dev {
                DeviceType::Shelly(shel) => found(shel),
                _ => {}
            }
        },
        not_found,
        (),
    );
}
