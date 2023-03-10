use crate::{devices, mutex_box::MutexBox};
use fireplace::eventhandler::{ActionType, EventType};
use rumqttc::{AsyncClient, QoS};
use tokio::time::{sleep, Duration};

pub static EVENT_HANDLER: MutexBox<Handler> = MutexBox::new("EventHandler");

pub struct Handler {
    client: AsyncClient,
    event_buffer: Vec<String>,
    action_buffer: Vec<ActionType>,
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
