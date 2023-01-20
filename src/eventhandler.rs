use std::{sync::Mutex,};

use rumqttc::{AsyncClient, QoS};

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

    pub async fn test_mqtt(&self) {
            self.client
                .publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![0x33; 9 as usize])
                .await
                .unwrap();
        
    }
}
