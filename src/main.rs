use actix_files as fs;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use home_center::{
    devices::SENSOR_LIST,
    eventhandler::{Handler, EVENT_HANDLER},
    mqtt,
};
use rumqttc::QoS;
use tokio::{self, task};

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
        if let Some(bla) =EVENT_HANDLER.lock().expect("msg").as_mut(){
            bla.test_mqtt().await;
        }
    }

    let http_handler = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(fs::Files::new("/", "./web/dist/").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))
    .unwrap()
    .run();

    //mqtt_work_task.await.unwrap();
    http_handler.await.unwrap();
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
