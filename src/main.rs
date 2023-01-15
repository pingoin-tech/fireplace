use chrono::Utc;
use rumqttc::QoS;
use std::{time::Duration, collections::HashMap};
use tokio::{self, task, time};
use home_center::{mqtt, devices::SENSOR_LIST};


#[tokio::main]
async fn main() {
    {
        SENSOR_LIST.lock().expect("could not lock").get_or_insert(HashMap::new());
    }

    let now = Utc::now();
    println!("{}", now);
    let (client, eventloop) = mqtt::init("rumqtt-async", "192.168.178.110", 1883);

    client
        .subscribe("shellies/#", QoS::AtMostOnce)
        .await
        .unwrap();

    let mqtt_work_task = task::spawn(mqtt::work(eventloop));
    for i in 0..10 {
        client
            .publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize])
            .await
            .unwrap();
        time::sleep(Duration::from_millis(100)).await;
    }
    mqtt_work_task.await.unwrap();
}
