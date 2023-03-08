use self::decodings::{
    decode_announce, decode_info, decode_other, decode_roller, decode_subdevice, decode_value,
};
use std::str::FromStr;
use ts_rs::TS;

use rumqttc::Publish;
use serde::{Deserialize, Serialize};

mod decodings;
mod incoming_data;

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
        "roller" => decode_roller(tel),
        "relay" => decode_subdevice(tel, "relay"),
        "light" => decode_subdevice(tel, "light"),
        "input" => decode_subdevice(tel, "input"),
        "info" => decode_info(tel),
        "voltage" => decode_value(tel, "voltage"),
        "temperature" => decode_value(tel, "temperature"),
        _ => {
            decode_other(tel);
        }
    }
}
