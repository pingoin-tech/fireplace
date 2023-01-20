use rumqttc::{
    AsyncClient,
    Event::{Incoming, Outgoing},
    EventLoop, MqttOptions,
    Packet, 
};
use rumqttc::mqttbytes::v4::Publish;
use std::{time::Duration};
use super::{shellies::decode_shelly_sub};


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
                Packet::Publish(content)=> {decode_subsciptions(content).await;},
                _ => {}
            },
            Outgoing(_) => {}
        }
    }
    ;
}

pub async fn decode_subsciptions(content:Publish){
    let mut path=content.topic.split("/");
    match path.next() {
        Some("shellies")=>{
            decode_shelly_sub(&content, path);
        },
        _=>{},
    }
}


