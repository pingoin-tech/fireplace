use std::{sync::Mutex};

use rumqttc::{AsyncClient, QoS};
use tokio::time::{sleep, Duration};

use crate::devices;

type EventHandler = Mutex<Option<Handler>>;

pub static EVENT_HANDLER: EventHandler = Mutex::new(None);

pub struct Handler {
    client: AsyncClient,
}

impl Handler {
    pub async fn new(client: AsyncClient) -> Self {
        let handle = Handler { client: client };
        return handle;
    }

    pub async fn test_mqtt(&mut self) {
        self.client
            .publish("shellies/command", QoS::AtLeastOnce, false, "announce")
            .await
            .unwrap();
        sleep(Duration::from_millis(1000)).await;
        self.force_action(String::from("schlafenEltern-lichtSchalter/announce"))
            .await;
    }

    pub async fn trigger_event(&self, event_string: String) {}

    pub async fn force_action(&mut self, action_string: String) -> bool {
        let (first, path) = split_action_string(action_string);
        let action = if let Some(id) = first {
            devices::get_device_from_list(
                id,
                |device| device.trigger_action(path),
                |_| ActionType::NotAvailable,
                ActionType::NotAvailable,
            )
        } else {
            ActionType::NotAvailable
        };

        match action {
            ActionType::NotAvailable => false,
            ActionType::MqttAction(topic, payload) => {
                match self
                    .client
                    .publish(topic, QoS::AtLeastOnce, false, payload)
                    .await
                {
                    Ok(_) => {
                        true
                    },
                    Err(err) => {
                        print!("{}", err);
                        false
                    }
                }
            }
        }
    }
}

pub fn split_action_string(action_string: String) -> (Option<String>, String) {
    let mut path = action_string.split("/");
    let mut first: Option<String> = None;
    if let Some(str_val) = path.next() {
        first = Some(String::from(str_val));
    }

    let mut result:String={path.map(|s| String::from(format!("{}/", s))).collect()};
    result.pop();

    (
        first,
        result,
    )
}

pub enum ActionType {
    NotAvailable,
    MqttAction(String, String),
}
