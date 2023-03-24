use crate::eventhandler::{ActionType, Event, EventName};

use serde::{Deserialize, Serialize};

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
    pub fn trigger_action(&mut self, action: Event) -> ActionType {
        let base_path = format!("shellies/{}/", action.id);
        println!("{:?}", action.event.clone());

        match action.event {
            EventName::Announce => {
                ActionType::MqttAction(format!("{}command", base_path), String::from("announce"))
            }
            EventName::Update => {
                ActionType::MqttAction(format!("{}command", base_path), String::from("update"))
            }
            EventName::On => {
                ActionType::MqttAction(format!("{}light/0/command", base_path), String::from("on"))
            }
            EventName::Off => {
                ActionType::MqttAction(format!("{}light/0/command", base_path), String::from("off"))
            }
            _ => ActionType::NotAvailable,
        }
    }
}
