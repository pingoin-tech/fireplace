use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
pub mod shellies;
use shellies::Shelly;
use ts_rs::TS;

use std::{collections::HashMap, sync::Mutex};

use crate::eventhandler::{ActionType, EventType, Value};

type DeviceDataBase = Mutex<Option<Vec<Device>>>;

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
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

#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
#[ts(export)]
pub enum DeviceType {
    Shelly(Shelly),
    Empty,
}

pub static SENSOR_LIST: DeviceDataBase = Mutex::new(None);

pub fn get_device_from_list<Fs, Ff, T>(id: String, found: Fs, not_found: Ff, error_val: T) -> T
where
    Fs: FnOnce(&mut Device) -> T,
    Ff: FnOnce(&mut Vec<Device>) -> T,
{
    if let Ok(mut list_option) = SENSOR_LIST.lock() {
        if let Some(list) = list_option.as_mut() {
            match list.into_iter().find(|x| x.id == id) {
                Some(device) => found(device),
                None => not_found(list),
            }
        } else {
            error_val
        }
    } else {
        error_val
    }
}

pub fn insert_value_in_device(id: String, key: String, val: Value) -> bool {
    get_device_from_list(
        id,
        |device| {
            device.values.insert(key, val);
            true
        },
        |_| false,
        false,
    )
}

impl Device {
    pub fn trigger_action(&mut self, action: EventType) -> ActionType {
        match &mut self.subdevice {
            DeviceType::Shelly(device) => device.trigger_action(action),
            DeviceType::Empty => ActionType::NotAvailable,
        }
    }
}
