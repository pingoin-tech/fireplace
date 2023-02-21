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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Shelly {
    pub fw_ver: String,
    pub shelly_type: ShellyType,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub rollers: Option<Vec<RollerStat>>,
    //pub update: UpdateStat,
    //pub meters: Vec<MeterStat>,
    //pub inputs: Vec<InputStat>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum ShellyType {
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

   pub fn from_announce(data: ShellyAnnounce) -> (Shelly, Vec<String>, Vec<String>) {
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
                // update: UpdateStat::default(),
                //meters: Vec::new(),
                //inputs: Vec::new(),
                //rollers: None,
            },
            actions,
            events,
        )
    }
}
