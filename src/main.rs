use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use home_center::{devices::SENSOR_LIST, mqtt};
use rumqttc::QoS;
use std::{collections::HashMap, time::Duration};
use tokio::{self, task, time};

#[tokio::main]
async fn main() {
    {
        SENSOR_LIST
            .lock()
            .expect("could not lock")
            .get_or_insert(HashMap::new());
    }

    let now = Utc::now();
    println!("{}", now);
    let (client, eventloop) = mqtt::init("rumqtt-async", "192.168.178.110", 1883);

    client
        .subscribe("shellies/#", QoS::AtMostOnce)
        .await
        .unwrap();

    task::spawn(mqtt::work(eventloop));

    let http_handler = HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 8080))
        .unwrap()
        .run();

    for i in 0..10 {
        client
            .publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize])
            .await
            .unwrap();
        time::sleep(Duration::from_millis(100)).await;
    }
    //mqtt_work_task.await.unwrap();
    http_handler.await.unwrap();
}

#[get("/")]
async fn hello() -> impl Responder {
    match SENSOR_LIST.lock() {
        Ok(mut list_option) => {
            if let Some(list) = list_option.as_mut() {
                return HttpResponse::Ok().json(list);
            }
        }
        Err(_) => {}
    }
    HttpResponse::Ok().body("bla")
}
