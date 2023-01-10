use rumqttc::{
    AsyncClient,
    Event::{Incoming, Outgoing},
    EventLoop, MqttOptions,
    Packet::Publish,
};
use std::time::Duration;

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
                Publish(content)=> println!("{}",content.topic),
                _ => {}
            },
            Outgoing(_) => {}
        }
    }
}
