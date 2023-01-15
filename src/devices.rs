use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::shellies::Shelly;
use std::{collections::HashMap, sync::Mutex};

type DeviceDataBase = Mutex<Option<HashMap<String, Device>>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub last_message: DateTime<Utc>,
    pub subdevice: DeviceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DeviceType {
    ShellyType(Shelly),
}

pub static SENSOR_LIST: DeviceDataBase = Mutex::new(None);
