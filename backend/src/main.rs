use std::time::Duration;

use actix_files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use backend::{
    devices::{init_sensor_list, SENSOR_LIST},
    eventhandler::{Handler, EVENT_HANDLER},
    mqtt,
    store::{init_store, STORE},
};
use fireplace::{config::Server, eventhandler::EventType};
use rumqttc::QoS;
use tokio::{self, task, time::sleep};

use git_version::git_version;
const GIT_VERSION: &str = git_version!(args = ["--always", "--tags"]);

#[tokio::main]
async fn main() {
    init_store();
    init_sensor_list();
    let mut mqtt_broker = Server::default();
    let mut http_server = Server::default();
    STORE.open_locked(
        |store| {
            mqtt_broker = store.config.mqtt_broker.clone();
            http_server = store.config.http_server.clone();
        },
        (),
    );

    let (client, eventloop) = mqtt::init("rumqtt-async", mqtt_broker.host, mqtt_broker.port);

    client
        .subscribe("shellies/#", QoS::AtMostOnce)
        .await
        .unwrap();

    EVENT_HANDLER.init(Handler::new(client).await);

    task::spawn(mqtt::work(eventloop));

    {
        if let Some(handler) = EVENT_HANDLER.mutex.lock().expect("locking failed").as_mut() {
            handler.init_devices().await;
        }
    }

    let http_handler = HttpServer::new(|| {
        App::new()
            .service(trigger_action)
            .service(devices)
            .service(version)
            .service(dev_setup)
            .service(links)
            .service(actix_files::Files::new("/", "./dist/").index_file("index.html"))
    })
    .bind((http_server.host, http_server.port))
    .unwrap()
    .run();

    let (_http, _) = tokio::join!(http_handler, event_handler_loop());
}

#[get("/api/devices")]
async fn devices() -> impl Responder {
    SENSOR_LIST.open_locked(
        |list| {
            list.sort_by(|a, b| b.id.cmp(&a.id));
            return HttpResponse::Ok().json(list);
        },
        HttpResponse::Ok().body("bla"),
    )
}

#[get("/api/links")]
async fn links() -> impl Responder {
    STORE.open_locked(
        |store| HttpResponse::Ok().json(&store.config.extra_links),
        HttpResponse::Ok().body("bla"),
    )
}

#[get("/api/device-setup")]
async fn dev_setup() -> impl Responder {
    STORE.open_locked(
        |store| HttpResponse::Ok().json(&store.config.device_settings),
        HttpResponse::Ok().body("bla"),
    )
}

#[get("/api/version")]
async fn version() -> impl Responder {
    HttpResponse::Ok().body(GIT_VERSION)
}

#[post("/api/trigger-action")]
async fn trigger_action(data: web::Json<EventType>) -> impl Responder {
    println!("{:?}", &data.0);

    let result = EVENT_HANDLER.open_locked(|handler| handler.force_action(data.0), false);
    if result {
        HttpResponse::Ok().body("true")
    } else {
        HttpResponse::Ok().body("false")
    }
}

async fn event_handler_loop() {
    loop {
        if let Some(handler) = EVENT_HANDLER.mutex.lock().expect("locking failed").as_mut() {
            handler.work().await;
        }
        sleep(Duration::from_millis(100)).await;
    }
}
