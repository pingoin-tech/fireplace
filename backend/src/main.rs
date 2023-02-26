use std::time::Duration;

use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use backend::{
    devices::SENSOR_LIST,
    eventhandler::{get_event_handler, Handler, EVENT_HANDLER},
    mqtt,
};
use fireplace::eventhandler::EventType;
use rumqttc::QoS;
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
// load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    let http_handler = HttpServer::new(|| {
        App::new()
            .service(trigger_action)
            .service(hello)
            .service(fs::Files::new("/", "./dist/").index_file("index.html"))
    })
    .bind_openssl(("0.0.0.0", 8080),builder)
    .unwrap()
    .run();

    let (_http, _) = tokio::join!(http_handler, event_handler_loop());
}

#[get("/api/devices")]
async fn hello() -> impl Responder {
    match SENSOR_LIST.lock() {
        Ok(mut list_option) => {
            if let Some(list) = list_option.as_mut() {
                list.sort_by(|a, b| b.id.cmp(&a.id));
                return HttpResponse::Ok().json(list);
            }
        }
        Err(_) => {}
    }
    HttpResponse::Ok().body("bla")
}

#[post("/api/trigger-action")]
async fn trigger_action(data: web::Json<EventType>) -> impl Responder {
    println!("{:?}", &data.0);

    let result = get_event_handler(|handler| handler.force_action(data.0), false);

    if result {
        HttpResponse::Ok().body("true")
    } else {
        HttpResponse::Ok().body("false")
    }
}

async fn event_handler_loop() {
    loop {
        if let Some(handler) = EVENT_HANDLER.lock().expect("locking failed").as_mut() {
            handler.work().await;
        }
        sleep(Duration::from_millis(100)).await;
    }
}
