use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
pub mod shellies; 
use shellies::Shelly;

use std::{collections::HashMap};

use super::eventhandler::{ActionType, EventType, Value};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Device {
    pub id: String,
    pub ip: String,
    pub last_message: DateTime<Utc>,
    pub subdevice: DeviceType,
    pub rssi: i16,
    pub available_actions: Vec<String>,
    pub available_events: Vec<String>,
    pub values: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Clone,Debug,PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DeviceType {
    Shelly(Shelly),
    Empty,
}

impl Device {
    pub fn trigger_action(&mut self, action: EventType) -> ActionType {
        match &mut self.subdevice {
            DeviceType::Shelly(device) => device.trigger_action(action),
            DeviceType::Empty => ActionType::NotAvailable,
        }
    }
}
