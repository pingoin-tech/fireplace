use crate::sensors::{self, EventAction};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Shelly {
    pub id: String,
    pub model: String,
    pub mac: String,
    pub ip: String,
    pub new_fw: bool,
    pub fw_ver: String,
    pub mode: Option<String>,
}

impl Shelly {}

impl sensors::Sensor for Shelly {
    fn get_actions(&self) -> Vec<EventAction> {
        Vec::new()
    }
    fn get_events(&self) -> Vec<EventAction> {
        Vec::new()
    }
}
