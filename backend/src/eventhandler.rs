use crate::{devices, mutex_box::MutexBox, store::STORE};
use chrono::Utc;
use fireplace::eventhandler::{ActionType, Event, EventName, EventType};
use rumqttc::{AsyncClient, QoS};
use tokio::time::{sleep, Duration};

pub static EVENT_HANDLER: MutexBox<Handler> = MutexBox::new("EventHandler");

pub struct Handler {
    client: AsyncClient,
    pub event_buffer: Vec<Event>,
}

impl Handler {
    pub async fn new(client: AsyncClient) -> Self {
        let handle = Handler {
            client: client,
            event_buffer: Vec::new(),
        };
        return handle;
    }

    pub async fn init_devices(&mut self) {
        self.client
            .publish("shellies/command", QoS::AtLeastOnce, false, "announce")
            .await
            .unwrap();
        sleep(Duration::from_millis(1000)).await;
        self.force_action(Event::new_action(
            &"schlafenEltern-lichtSchalter".to_string(),
            EventName::Announce,
        ));
    }

    pub fn trigger_event(&mut self, event: Event) {
        self.event_buffer.push(event);
    }

    pub fn force_action(&mut self, action_triggered: Event) -> bool {
        self.event_buffer.push(action_triggered.clone());
        true
    }

    pub async fn work(&mut self) {
        let now = Utc::now();
        self.event_buffer.retain(|event| {
            let result = now - event.timestamp <= chrono::Duration::seconds(600);
            result
        });
        let mut tmp_event_buffer = self.event_buffer.clone();

        for event in tmp_event_buffer.iter_mut() {
            if !event.handled {
                match event.event_type {
                    fireplace::eventhandler::EventType::Event => self.work_event(event).await,
                    fireplace::eventhandler::EventType::Action => self.work_action(event).await,
                }
            }
        }
        self.event_buffer = tmp_event_buffer;
    }

    async fn work_action(&mut self, event: &mut Event) {
        let action = devices::get_device_from_list(
            event.id.clone(),
            |device| device.trigger_action(event),
            |_| ActionType::NotAvailable,
            ActionType::NotAvailable,
        );
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
        event.handled = true;
    }

    async fn work_event(&mut self, event: &mut Event) {
        let mut actions: Vec<Event> = Vec::new();
        STORE.open_locked(
            |store| {
                store.config.actions.iter().for_each(|action| {
                    if action.event_is_equal(&event) {
                        let mut new_action = action.action.clone();
                        new_action.handled = false;
                        new_action.event_type = EventType::Action;
                        new_action.timestamp = Utc::now();
                        dbg!(&new_action);
                        actions.push(new_action);
                    }
                })
            },
            (),
        );
        for mut ac in actions{
            self.work_action(&mut ac).await;
        }
        match event.event {
            EventName::NewData => (),
            EventName::InputShort => {
                dbg!(&event);
            }
            EventName::InputLong => {
                dbg!(&event);
            }
            _ => {
                dbg!(&event);
            }
        }
        event.handled = true;
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
