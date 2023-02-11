use self::{
    decodings::{decode_other, decode_subdevice, decode_value},
    incoming_data::{InputStat, LightStat, MeterStat, RelaysState, RollerStat, UpdateStat},
};
use crate::eventhandler::{ActionType, EventType};
use std::str::FromStr;
use ts_rs::TS;

use rumqttc::Publish;
use serde::{Deserialize, Serialize};

mod decodings;
mod incoming_data;
use decodings::{decode_announce, decode_info};
use incoming_data::ShellyAnnounce;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct Shelly {
    pub fw_ver: String,
    pub shelly_type: ShellyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relays: Option<Vec<RelaysState>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lights: Option<Vec<LightStat>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollers: Option<Vec<RollerStat>>,
    pub update: UpdateStat,
    pub meters: Vec<MeterStat>,
    pub inputs: Vec<InputStat>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, TS)]
#[ts(export)]
pub enum ShellyType {
    Shelly1,
    ShellyDimmer,
    Shelly25Roller,
    Shelly25Switch,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct Telegram {
    pub id: String,
    pub subdevice: Option<String>,
    pub subdevice_number: Option<usize>,
    pub topic: String,
    pub payload: String,
}

impl Shelly {
    pub fn trigger_action(&mut self, action: EventType) -> ActionType {
        let base_path = format!("shellies/{}/", action.id);
        println!("{}", action.action.clone());

        match action.action.as_str() {
            "announce" => {
                ActionType::MqttAction(format!("{}command", base_path), String::from("announce"))
            }
            "update" => {
                ActionType::MqttAction(format!("{}command", base_path), String::from("update"))
            }
            "on" => {
                ActionType::MqttAction(format!("{}light/0/command", base_path), String::from("on"))
            }
            "off" => {
                ActionType::MqttAction(format!("{}light/0/command", base_path), String::from("off"))
            }
            _ => ActionType::NotAvailable,
        }
    }

    fn from_announce(data: ShellyAnnounce) -> (Shelly, Vec<String>, Vec<String>) {
        let mut shelly_type = ShellyType::Shelly1;

        let mut actions = vec!["announce".to_string(), "update".to_string()];
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
                actions.push("on".to_string());
                actions.push("off".to_string());
            }
            _ => {}
        }

        (
            Shelly {
                fw_ver: data.fw_ver,
                shelly_type: shelly_type,
                relays: None,
                update: UpdateStat::default(),
                meters: Vec::new(),
                inputs: Vec::new(),
                lights: None,
                rollers: None,
            },
            actions,
            events,
        )
    }
}

pub fn decode_shelly_sub(content: &Publish) {
    let topic = content.topic.split("/");

    let mut topic_list = Vec::new();
    topic.for_each(|val| topic_list.push(val.to_string()));
    let payload = String::from_utf8((&content.payload).to_vec()).unwrap();
    let tel = match topic_list.len() {
        3 => Telegram {
            id: topic_list[1].clone(),
            subdevice: None,
            subdevice_number: None,
            topic: topic_list[2].clone(),
            payload: payload,
        },
        4 => {
            let index = usize::from_str(topic_list[3].clone().as_str());
            if let Ok(index) = index {
                Telegram {
                    id: topic_list[1].clone(),
                    subdevice: None,
                    subdevice_number: Some(index),
                    topic: topic_list[2].clone(),
                    payload: payload,
                }
            } else {
                Telegram {
                    id: topic_list[1].clone(),
                    subdevice: Some(topic_list[3].clone()),
                    subdevice_number: None,
                    topic: topic_list[2].clone(),
                    payload: payload,
                }
            }
        }
        5 => {
            let index = usize::from_str(topic_list[3].clone().as_str());
            if let Ok(index) = index {
                Telegram {
                    id: topic_list[1].clone(),
                    subdevice: Some(topic_list[4].clone()),
                    subdevice_number: Some(index),
                    topic: topic_list[2].clone(),
                    payload: payload,
                }
            } else {
                Telegram {
                    id: topic_list[1].clone(),
                    subdevice: Some(topic_list[3].clone()),
                    subdevice_number: None,
                    topic: topic_list[2].clone(),
                    payload: payload,
                }
            }
        }
        _ => Telegram {
            id: "".to_string(),
            subdevice: None,
            subdevice_number: None,
            topic: topic_list[1].clone(),
            payload: payload,
        },
    };

    match tel.topic.as_str() {
        "announce" => decode_announce(tel),
        "command" => {}
        "online" => {}
        "temperature_f" => {}
        "overtemperature" => {}
        "overpower" => {}
        "loaderror" => {}
        "temperature_status" => {}
        "relay" => decode_subdevice(tel, "relay"),
        "light" => decode_subdevice(tel, "light"),
        "info" => decode_info(tel),
        "voltage" => decode_value(tel, "voltage"),
        "temperature" => decode_value(tel, "temperature"),
        _ => {
            decode_other(tel);
        }
    }
}
