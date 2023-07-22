use crate::eventhandler::{ActionType, Event, EventName, Value};
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::subdevices::SubDevice;

#[derive(Serialize, Deserialize, Debug)]
pub struct ShellyAnnounce {
    pub id: String,
    pub model: String,
    pub mac: String,
    pub ip: String,
    pub new_fw: bool,
    pub fw_ver: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(tag = "subtype", rename_all = "snake_case")]
pub enum Shelly {
    Shelly1,
    ShellyDimmer,
    Shelly25Roller,
    Shelly25Switch,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Telegram {
    pub id: String,
    pub subdevice: Option<String>,
    pub subdevice_number: Option<usize>,
    pub topic: String,
    pub payload: String,
}

impl Shelly {
    pub fn trigger_action(
        &mut self,
        action: &Event,
        values: BTreeMap<String, SubDevice>,
    ) -> ActionType {
        let base_path = format!("shellies/{}/", action.id);
        let index = match action.index {
            Some(index) => index,
            None => 0,
        };

        match action.event {
            EventName::Announce => {
                ActionType::MqttAction(format!("{}command", base_path), String::from("announce"))
            }
            EventName::Update => {
                ActionType::MqttAction(format!("{}command", base_path), String::from("update"))
            }
            EventName::On => match self {
                Shelly::Shelly1 => ActionType::MqttAction(
                    format!("{}relay/{}/command", base_path, index),
                    "on".to_string(),
                ),
                Shelly::ShellyDimmer => ActionType::MqttAction(
                    format!("{}light/{}/command", base_path, 0),
                    "on".to_string(),
                ),
                Shelly::Shelly25Roller => ActionType::NotAvailable,
                Shelly::Shelly25Switch => ActionType::MqttAction(
                    format!("{}relay/{}/command", base_path, index),
                    "on".to_string(),
                ),
            },
            EventName::Off => match self {
                Shelly::Shelly1 => ActionType::MqttAction(
                    format!("{}relay/{}/command", base_path, index),
                    "off".to_string(),
                ),
                Shelly::ShellyDimmer => ActionType::MqttAction(
                    format!("{}light/{}/command", base_path, 0),
                    "off".to_string(),
                ),
                Shelly::Shelly25Roller => ActionType::NotAvailable,
                Shelly::Shelly25Switch => ActionType::MqttAction(
                    format!("{}relay/{}/command", base_path, index),
                    "off".to_string(),
                ),
            },
            EventName::Toggle => match self {
                Shelly::Shelly1 => ActionType::MqttAction(
                    format!("{}relay/{}/command", base_path, index),
                    "toggle".to_string(),
                ),
                Shelly::ShellyDimmer => {
                    let toggle = match values.get("light-on") {
                        Some(is_on) => match is_on {
                            SubDevice::Sensor(Value::Bool(is_on) )=> !!is_on,
                            _ => false,
                        },
                        None => false,
                    };
                    let toggle = if toggle { "off" } else { "on" };
                    ActionType::MqttAction(
                        format!("{}light/{}/command", base_path, 0),
                        toggle.to_string(),
                    )
                }
                Shelly::Shelly25Roller => {
                    let toggle = {
                        if let Some(SubDevice::Sensor(Value::String(status)) )= values.get("roller-status") {
                            if status.as_str() != "stop" {
                                "stop"
                            } else {
                                if let Some(SubDevice::Sensor(Value::String(last_dir)) )=
                                    values.get("roller-last-direction")
                                {
                                    if last_dir.as_str() == "open" {
                                        "close"
                                    } else {
                                        "open"
                                    }
                                } else {
                                    if let Some(SubDevice::Sensor(Value::Number(pos))) = values.get("roller-position")
                                    {
                                        if pos.abs() > 50.0 {
                                            "close"
                                        } else {
                                            "open"
                                        }
                                    } else {
                                        "open"
                                    }
                                }
                            }
                        } else {
                            "open"
                        }
                    };
                    ActionType::MqttAction(
                        format!("{}roller/{}/command", base_path, index),
                        toggle.to_string(),
                    )
                }
                Shelly::Shelly25Switch => ActionType::MqttAction(
                    format!("{}relay/{}/command", base_path, index),
                    "toggle".to_string(),
                ),
            },
            EventName::Open => match self {
                Shelly::Shelly25Roller => ActionType::MqttAction(
                    format!("{}roller/{}/command", base_path, index),
                    "open".to_string(),
                ),
                _ => ActionType::NotAvailable,
            },
            EventName::Close => match self {
                Shelly::Shelly25Roller => ActionType::MqttAction(
                    format!("{}roller/{}/command", base_path, index),
                    "close".to_string(),
                ),
                _ => ActionType::NotAvailable,
            },
            EventName::Stop => match self {
                Shelly::Shelly25Roller => ActionType::MqttAction(
                    format!("{}roller/{}/command", base_path, index),
                    "stop".to_string(),
                ),
                _ => ActionType::NotAvailable,
            },
            _ => ActionType::NotAvailable,
        }
    }
}
