use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
pub mod shellies;
use shellies::Shelly;

use std::sync::Mutex;

type DeviceDataBase = Mutex<Option<Vec<Device>>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub id: String,
    pub ip: String,
    pub last_message: DateTime<Utc>,
    pub subdevice: DeviceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DeviceType {
    Shelly(Shelly),
    Empty,
}

pub static SENSOR_LIST: DeviceDataBase = Mutex::new(None);

pub fn get_device_from_list<Fs, Ff>(id: String, found: Fs, not_found: Ff)
where
    Fs: FnOnce(&mut Device),
    Ff: FnOnce(&mut Vec<Device>),
{
    if let Ok(mut list_option) = SENSOR_LIST.lock() {
        if let Some(list) = list_option.as_mut() {
            match list.into_iter().find(|x| x.id == id) {
                Some(device) => found(device),
                None => not_found(list),
            }
        }
    }
}
