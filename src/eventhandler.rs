use serde::{Deserialize, Serialize};
use std::fmt::{self,Display};

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
            Value::Bool(val)=>{write!(f,"{}",val)},
            Value::Number(val)=>{write!(f,"{}",val)},
            Value::String(val)=>{write!(f,"{}",val)},
        }
     }
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct EventType {
    pub id: String,
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdevice: Option<String>,
}

#[derive(PartialEq, Clone)]
pub enum ActionType {
    NotAvailable,
    MqttAction(String, String),
}
