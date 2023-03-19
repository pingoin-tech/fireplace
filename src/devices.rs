use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
pub mod shellies;
use shellies::Shelly;

use std::collections::BTreeMap;

use super::eventhandler::{ActionType, EventType, Value};

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
    pub available_actions: Vec<EventType>,
    pub available_events: Vec<String>,
    pub values: BTreeMap<String, Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DeviceType {
    Shelly(Shelly),
    Empty,
}

impl Default for DeviceType {
    fn default() -> Self {
        DeviceType::Empty
    }
}

impl Device {
    pub fn trigger_action(&mut self, action: EventType) -> ActionType {
        match &mut self.subdevice {
            DeviceType::Shelly(device) => device.trigger_action(action),
            DeviceType::Empty => ActionType::NotAvailable,
        }
    }
}
