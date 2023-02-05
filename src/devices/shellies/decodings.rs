use std::{str, str::Split};

use chrono::Utc;
use rumqttc::Publish;
use serde_json::Error;
use std::str::FromStr;

use crate::{devices::{get_device_from_list, Device, DeviceType}, eventhandler::get_event_handler};

use super::{
    incoming_data::{ShellyAnnounce, ShellyInfo},
    Shelly,
};

pub fn decode_announce(content: &Publish) {
    let dev_res: Result<ShellyAnnounce, Error> = serde_json::from_slice(&content.payload);
    match dev_res {
        Ok(device) => get_device_from_list(
            device.id.clone(),
            |dev| {
                dev.last_message = Utc::now();
            },
            |list| {
                let id = device.id.clone();
                let ip = device.ip.clone();
                let (shelly,actions,events)=Shelly::from_announce(device);
                let sub_device = DeviceType::Shelly(shelly);
                list.push(Device {
                    id: id,
                    ip: ip,
                    last_message: Utc::now(),
                    subdevice: sub_device,
                    available_actions:actions,
                    available_events: events,
                    rssi:0,
                });
            },
            ()
        ),
        Err(err) => println!("{:?}", err),
    }
}

pub fn decode_info(content: &Publish, id: String) {
    let dev_res: Result<ShellyInfo, Error> = serde_json::from_slice(&content.payload);
    match dev_res {
        Ok(info_data) => {
            get_device_from_list(id.clone(),|device|{
                device.rssi=info_data.wifi_sta.rssi;
                match &mut device.subdevice {
                    DeviceType::Shelly(shel)=>{
                        shel.inputs = info_data.inputs;
                        shel.meters = info_data.meters;
                        shel.relays = info_data.relays;
                        shel.update = info_data.update;
                        shel.uptime = info_data.uptime;
                        shel.lights = info_data.lights;
                        shel.rollers = info_data.rollers;
                        shel.overpower = info_data.overpower;
                        shel.overtemperature = info_data.overtemperature;
                    },
                    _=>{}
                };


            },|_|{()},());
        }
        Err(err) => println!("{:?}", err),
    }
}

pub fn decode_other(path: &str, id: String, content: &Publish) {
    get_device_from_list(
        id.clone(),
        |dev| {
            dev.last_message = Utc::now();
            println!("State input: {}/{}: {:?}", dev.id, path, content.payload);
        },
        |_| println!("Unknown device: {}/{}: {:?}", id, path, content.payload),
        ()
    );
}

pub fn decode_voltage(content: &Publish, id: String){
    if let Ok(string) = str::from_utf8(&content.payload) {
        if let Ok(val) = f32::from_str(string){
            super::open_shelly_fom_list(id, |shelly|{
                shelly.voltage=Some(val);
            }, |_|{})
        }
    }
}

pub fn decode_relay(content: &Publish, id: String, mut path: Split<&str>) {
    let index_op: Option<usize> = match path.next() {
        None => None,
        Some("0") => Some(0),
        Some("1") => Some(1),
        _ => None,
    };
    if let Some(index) = index_op {
        let lower = str::from_utf8(&content.payload);
        super::open_shelly_fom_list(
            id.clone(),
            |shelly| {
                if let Some(relay_arr) = &mut shelly.relays {
                    if let Some(relay) = relay_arr.get_mut(index) {
                        match lower {
                            Ok("on") => {
                                relay.ison = true;
                                relay.overpower = Some(false);
                            }
                            Ok("off") => {
                                relay.ison = false;
                                relay.overpower = Some(false);
                            }
                            Ok("overpower") => {
                                relay.ison = false;
                                relay.overpower = Some(true);
                            }
                            _ => {}
                        };
                    }
                }
            },
            |_| {},
        );
    }
    trigger_new_data(id)
}


fn trigger_new_data(id: String){
    get_event_handler(|handler|{
        handler.trigger_event(format!("{}/new_data",id))
    },())
}