use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
pub mod shellies;
use shellies::Shelly;

use std::collections::BTreeMap;

use super::eventhandler::{ActionType, Event, Value};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Device {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    pub id: String,
    pub ip: String,
    pub mac: String,
    pub last_message: DateTime<Utc>,
    pub last_data: DateTime<Utc>,
    pub subdevice: DeviceType,
    pub rssi: i16,
    pub available_actions: Vec<Event>,
    pub available_events: Vec<Event>,
    pub values: BTreeMap<String, Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DeviceType {
    Shelly(Shelly),
    WeatherUndergrundDevice,
    Empty,
}

impl Default for DeviceType {
    fn default() -> Self {
        DeviceType::Empty
    }
}

impl DeviceType {
    pub fn from_string(string: &String) -> Self {
        let input = string.to_lowercase();
        match input.as_str() {
            _ => DeviceType::Empty,
        }
    }
}

impl Device {
    pub fn trigger_action(&mut self, action: &Event) -> ActionType {
        let vals = self.values.clone();
        match &mut self.subdevice {
            DeviceType::Shelly(device) => device.trigger_action(action, vals),
            DeviceType::Empty => {
                println!("{:?}", action);
                ActionType::NotAvailable
            }
            DeviceType::WeatherUndergrundDevice => {
                println!("{:?}", action);
                ActionType::NotAvailable
            }
        }
    }
}
