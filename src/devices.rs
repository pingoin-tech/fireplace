use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::shellies::{Shelly1, Shelly25Roller, ShellyDimmer};

use std::{ sync::Mutex};

type DeviceDataBase = Mutex<Option<Vec<Device>>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub id:String,
    pub last_message: DateTime<Utc>,
    pub subdevice: DeviceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DeviceType {
    Shelly1Type(Shelly1),
    Shelly25RollerType(Shelly25Roller),
    ShellyDimmerType(ShellyDimmer),
    Empty,
}

pub static SENSOR_LIST: DeviceDataBase = Mutex::new(None);
