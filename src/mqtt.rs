use rumqttc::{MqttOptions, AsyncClient, QoS};
use std::time::Duration;

use crate::configuration::Mqtt;

async fn run(configuration: Mqtt) {
    let mut mqtt_options: MqttOptions = MqttOptions::new(
        &configuration.client_id,
        &configuration.host,
        configuration.port
    );
    mqtt_options.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);
    client.subscribe("hello/rumqtt", QoS::AtMostOnce).await.unwrap();

    while let Ok(notification) = eventloop.poll().await {
        println!("Received = {:?}", notification);
    }
}
