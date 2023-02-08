use std::sync::Mutex;

use rumqttc::{AsyncClient, QoS};
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
use ts_rs::TS;

use crate::devices;

type EventHandler = Mutex<Option<Handler>>;

pub static EVENT_HANDLER: EventHandler = Mutex::new(None);

pub struct Handler {
    client: AsyncClient,
    event_buffer: Vec<String>,
    action_buffer: Vec<ActionType>,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(tag = "type", content = "val", rename_all = "snake_case")]
#[ts(export)]
pub enum Value {
    Number(f32),
    Bool(bool),
    String(String),
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub struct EventType {
    pub id: String,
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdevice: Option<String>,
}

impl Handler {
    pub async fn new(client: AsyncClient) -> Self {
        let handle = Handler {
            client: client,
            event_buffer: Vec::new(),
            action_buffer: Vec::new(),
        };
        return handle;
    }

    pub async fn init_devices(&mut self) {
        self.client
            .publish("shellies/command", QoS::AtLeastOnce, false, "announce")
            .await
            .unwrap();
        sleep(Duration::from_millis(1000)).await;
        self.force_action(EventType {
            id: "schlafenEltern-lichtSchalter".to_string(),
            action: "announce".to_string(),
            value: None,
            subdevice: None,
        });
    }

    pub fn trigger_event(&mut self, event_string: String) {
        self.event_buffer.push(event_string);
    }

    pub fn force_action(&mut self, action_triggered: EventType) -> bool {
        let action = devices::get_device_from_list(
            action_triggered.id.clone(),
            |device| device.trigger_action(action_triggered),
            |_| ActionType::NotAvailable,
            ActionType::NotAvailable,
        );

        self.action_buffer.push(action.clone());

        if action == ActionType::NotAvailable {
            false
        } else {
            true
        }
    }

    async fn work_action(&mut self) {
        if let Some(action) = self.action_buffer.pop() {
            match action {
                ActionType::NotAvailable => {}
                ActionType::MqttAction(topic, payload) => {
                    match self
                        .client
                        .publish(topic, QoS::AtLeastOnce, false, payload)
                        .await
                    {
                        Ok(_) => {}
                        Err(err) => {
                            print!("{}", err);
                        }
                    }
                }
            }
        }
    }

    pub async fn work(&mut self) {
        self.event_buffer.pop();
        self.work_action().await;
    }
}

pub fn split_action_string(action_string: String) -> (Option<String>, String) {
    let mut path = action_string.split("/");
    let mut first: Option<String> = None;
    if let Some(str_val) = path.next() {
        first = Some(String::from(str_val));
    }

    let mut result: String = { path.map(|s| String::from(format!("{}/", s))).collect() };
    result.pop();

    (first, result)
}

#[derive(PartialEq, Clone)]
pub enum ActionType {
    NotAvailable,
    MqttAction(String, String),
}

pub fn get_event_handler<Fs, T>(found: Fs, error_val: T) -> T
where
    Fs: FnOnce(&mut Handler) -> T,
{
    if let Ok(mut handler_option) = EVENT_HANDLER.lock() {
        if let Some(handler) = handler_option.as_mut() {
            found(handler)
        } else {
            error_val
        }
    } else {
        error_val
    }
}
