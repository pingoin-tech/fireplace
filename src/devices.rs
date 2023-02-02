use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
pub mod shellies;
use shellies::Shelly;

use std::{sync::Mutex};

use crate::eventhandler::ActionType;

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

pub fn get_device_from_list<Fs, Ff,T>(id: String, found: Fs, not_found: Ff,error_val:T)->T
where
    Fs: FnOnce(&mut Device)->T,
    Ff: FnOnce(&mut Vec<Device>)->T,
{
    if let Ok(mut list_option) = SENSOR_LIST.lock() {
        if let Some(list) = list_option.as_mut() {
            match list.into_iter().find(|x| x.id == id) {
                Some(device) => found(device),
                None => not_found(list),
            }
        }else{
            error_val
        }
    }else{
        error_val
    }
}


impl Device {
    pub fn trigger_action(&mut self,action_path:String)->ActionType{
        match &mut self.subdevice {
            DeviceType::Shelly(device) => {
                device.trigger_action(action_path, self.id.clone())
            },
            DeviceType::Empty => ActionType::NotAvailable,
        }
    }  
}

