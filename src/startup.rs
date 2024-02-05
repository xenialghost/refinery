use rumqttc::{MqttOptions, AsyncClient, Event, Incoming, QoS};
use std::time::Duration;
use rmp_serde::{encode, decode};
use rmp_serde::{Deserializer, Serializer};
use serde::{Serialize, Deserialize};

use crate::configuration::Settings;

// #[derive(Serialize, Deserialize, Debug)]
// struct Message {
//     name: String,
// }

// impl From<&Message> for Vec<u8> {
//     fn from(value: &Message) -> Self {
//         encode::to_vec(value).unwrap();
//     }
// }

// impl TryFrom<&[u8]> for Message {
//     type Error = Box<decode::Error>;

//     fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
//         decode::from_slice();
//     }
// }


pub async fn run(configuration: Settings) {
    let mut mqtt_options: MqttOptions = MqttOptions::new(
        &configuration.mqtt.client_id,
        &configuration.mqtt.host,
        configuration.mqtt.port
    );
    mqtt_options
        .set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);
    client
        .subscribe("hello/rumqtt", QoS::AtMostOnce)
        .await
        .unwrap();

    while let Ok(event) = eventloop.poll().await {
        if let Event::Incoming(Incoming::Publish(packet)) = event {
            // println!("Received = {:?}", packet);
            // match Message::try_from(packet.payload.as_ref()) {
            //     Ok(message) => println!("Payload = {message:?}"),
            //     Err(error) => println!("Error = {error}"),
            // }
            println!("Received = {:?}", packet.payload.as_ref());

        }
    }
}
