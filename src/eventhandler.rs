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
    /// ID of the device
    pub id: String,
    /// which event is treiggered
    pub event: EventName,
    /// time when event is triggered
    pub timestamp: DateTime<Utc>,
    /// wether it is an action or an event
    pub event_type: EventType,
    /// Wheter it is handled yet
    pub handled: bool,
    /// Subdevice which is concerned
    /// None when main device is concerned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdevice: Option<String>,
}

impl Event {
    pub fn new_action(id: &String, event: EventName) -> Self {
        Self {
            id: id.clone(),
            event: event,
            timestamp: Utc::now(),
            event_type: EventType::Action,
            handled: false,
            subdevice: None,
        }
    }

    pub fn new_event(id: &String, event: EventName) -> Self {
        Self {
            id: id.clone(),
            event: event,
            timestamp: Utc::now(),
            event_type: EventType::Event,
            handled: false,
            subdevice: None,
        }
    }
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EventType {
    Event,
    Action,
}

#[derive(PartialEq, Clone)]
pub enum ActionType {
    NotAvailable,
    MqttAction(String, String),
}
