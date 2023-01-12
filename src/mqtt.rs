use rumqttc::{
    AsyncClient,
    Event::{Incoming, Outgoing},
    EventLoop, MqttOptions,
    Packet, 
};
use rumqttc::mqttbytes::v4::Publish;
use std::{time::Duration, str::Split};
use super::shellies::Shelly;

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
                Packet::Publish(content)=> decode_subsciptions(content),
                _ => {}
            },
            Outgoing(_) => {}
        }
    }
}

pub fn decode_subsciptions(content:Publish){
    let mut path=content.topic.split("/");
    match path.next() {
        Some("shellies")=>{
            decode_shelly_sub(&content, path)
        },
        _=>{},
    }
}

pub fn decode_shelly_sub(content:&Publish,mut path:Split<&str>){
    match path.next() {
        Some("announce")=>{

            match serde_json::from_slice(&content.payload) {
                Ok(device) => {
                    let shelly_device:Shelly=device;
                    println!("{:?}", shelly_device);
                },
                Err(err) => println!("{:?}",err),
            }
            
            
        },
        _=>{}, 
    }
}
