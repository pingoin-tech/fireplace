use crate::eventhandler::{ActionType, EventType};

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
}
