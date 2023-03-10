use super::devices::shellies::decode_shelly_sub;
use rumqttc::mqttbytes::v4::Publish;
use rumqttc::{
    AsyncClient,
    Event::{Incoming, Outgoing},
    EventLoop, MqttOptions, Packet,
};
use std::time::Duration;
use tokio::task;

pub fn init<S, T>(id: S, host: T, port: u16) -> (AsyncClient, EventLoop)
where
    S: Into<String>,
    T: Into<String>,
{
    let mut mqttoptions = MqttOptions::new(id, host, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, eventloop) = AsyncClient::new(mqttoptions, 10);

    (client, eventloop)
}

pub async fn work(mut eventloop: EventLoop) {
    while let Ok(notification) = eventloop.poll().await {
        match notification {
            Incoming(pack) => match pack {
                Packet::Publish(content) => {
                    task::spawn(decode_subsciptions(content));
                }
                _ => {}
            },
            Outgoing(_) => {}
        }
    }
}

pub async fn decode_subsciptions(content: Publish) {
    let mut path = content.topic.split("/");
    match path.next() {
        Some("shellies") => {
            decode_shelly_sub(&content);
        }
        _ => {}
    }
}
