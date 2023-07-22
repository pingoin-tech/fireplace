use serde::{Deserialize, Serialize};

use crate::eventhandler::Value;

pub mod relays;

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub enum SubDevice {
    Relays(relays::Relays),
    Sensor(Value),
    #[default]
    Empty,
}
