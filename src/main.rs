use rumqttc::QoS;
use std::time::Duration;
use tokio::{self, task, time};

mod mqtt;

#[tokio::main]
async fn main() {
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
