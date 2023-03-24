use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "type", content = "val", rename_all = "snake_case")]
pub enum Value {
    Number(f32),
    Bool(bool),
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Bool(val) => {
                write!(f, "{}", val)
            }
            Value::Number(val) => {
                write!(f, "{}", val)
            }
            Value::String(val) => {
                write!(f, "{}", val)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Event {
    ///ID of the device
    pub id: String,
    pub event: EventName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdevice: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EventName {
    NewData,
    InputShort,
    InputLong,
    On,
    Off,
    Toggle,
    Update,
    Announce,
}

impl fmt::Display for EventName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Clone)]
pub enum ActionType {
    NotAvailable,
    MqttAction(String, String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TimedEvent {
    pub timestamp: DateTime<Utc>,
    pub event: Event,
}
