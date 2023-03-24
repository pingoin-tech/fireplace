use crate::{devices, mutex_box::MutexBox};
use chrono::Utc;
use fireplace::eventhandler::{ActionType, Event, TimedEvent, EventName};
use rumqttc::{AsyncClient, QoS};
use tokio::time::{sleep, Duration};

pub static EVENT_HANDLER: MutexBox<Handler> = MutexBox::new("EventHandler");

pub struct Handler {
    client: AsyncClient,
    event_buffer: Vec<Event>,
    action_buffer: Vec<ActionType>,
    pub last_events: Vec<TimedEvent>,
    pub last_actions: Vec<TimedEvent>,
}

impl Handler {
    pub async fn new(client: AsyncClient) -> Self {
        let handle = Handler {
            client: client,
            event_buffer: Vec::new(),
            action_buffer: Vec::new(),
            last_actions: Vec::new(),
            last_events: Vec::new(),
        };
        return handle;
    }

    pub async fn init_devices(&mut self) {
        self.client
            .publish("shellies/command", QoS::AtLeastOnce, false, "announce")
            .await
            .unwrap();
        sleep(Duration::from_millis(1000)).await;
        self.force_action(Event {
            id: "schlafenEltern-lichtSchalter".to_string(),
            event:EventName::Announce,
            value: None,
            subdevice: None,
        });
    }

    pub fn trigger_event(&mut self, event: Event) {
        self.last_events.push(TimedEvent {
            event: event.clone(),
            timestamp: Utc::now(),
        });

        self.event_buffer.push(event);
    }

    pub fn force_action(&mut self, action_triggered: Event) -> bool {
        self.last_actions.push(TimedEvent {
            timestamp: Utc::now(),
            event: action_triggered.clone(),
        });

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
        let now = Utc::now();
        self.last_events.retain(|event| {
            let result = now - event.timestamp <= chrono::Duration::seconds(20);
            result
        });
        self.last_actions
            .retain(|event| now - event.timestamp <= chrono::Duration::seconds(20));

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
