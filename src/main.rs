use std::time::Duration;

use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use home_center::{
    devices::SENSOR_LIST,
    eventhandler::{get_event_handler, Handler, EVENT_HANDLER},
    mqtt,
};
use rumqttc::QoS;
use serde::Deserialize;
use tokio::{self, task, time::sleep};

#[tokio::main]
async fn main() {
    {
        SENSOR_LIST
            .lock()
            .expect("could not lock")
            .get_or_insert(Vec::new());
    }

    let (client, eventloop) = mqtt::init("rumqtt-async", "192.168.178.110", 1883);

    client
        .subscribe("shellies/#", QoS::AtMostOnce)
        .await
        .unwrap();

    {
        EVENT_HANDLER
            .lock()
            .expect("could not lock")
            .get_or_insert(Handler::new(client).await);
    }

    task::spawn(mqtt::work(eventloop));

    {
        if let Some(handler) = EVENT_HANDLER.lock().expect("locking failed").as_mut() {
            handler.init_devices().await;
        }
    }

    let http_handler = HttpServer::new(|| {
        App::new()
            .service(trigger_action)
            .service(hello)
            .service(fs::Files::new("/", "./web/dist/").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))
    .unwrap()
    .run();


   let (_http,_)= tokio::join!(http_handler,event_handler_loop());
}

#[get("/api/devices/")]
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

#[derive(Deserialize, Debug)]
struct Action {
    action_string: String,
}

#[post("/api/trigger-action/")]
async fn trigger_action(data: web::Json<Action>) -> impl Responder {
    let result = get_event_handler(
        |handler| handler.force_action(data.action_string.clone()),
        false,
    );

    if result {
        HttpResponse::Ok().body("true")
    } else {
        HttpResponse::Ok().body("false")
    }
}

async fn event_handler_loop(){
    loop {
        if let Some(handler) = EVENT_HANDLER.lock().expect("locking failed").as_mut() {
            handler.work().await;
        }
        sleep(Duration::from_millis(100)).await;
    }
}
