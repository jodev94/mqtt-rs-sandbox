// TODO: Remove this when dev is complete
#![allow(unused_mut, unused_variables)]

use chrono::{DateTime, Utc};
use rumqttc::{Client, MqttOptions, QoS};
use std::{collections::HashMap, thread, time::Duration};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct payload {
    ts: String,
    data: HashMap<String, String>,
}

impl payload {

    fn build_fake_data() -> payload {
        return payload {
            ts: Utc::now().to_string(),
            data: HashMap::from([
                // TODO: make this sensor data random
                ("sen0".to_string(), "123.4".to_string()),
                ("sen2".to_string(), "456.7".to_string()),
             ])     
        };
    }

    fn payload_to_json_string(pld: &payload) -> String{

        let json = serde_json::to_string_pretty(pld).unwrap();
        return json;
    }
}


fn mqtt() {
    // Running mosquitto on Docker locally
    let mut mqttoptions = MqttOptions::new("rumqtt-sync", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));


    dbg!("Running MQTT Svc");

    let (mut client, mut connection) = Client::new(mqttoptions, 10);


    thread::spawn(move || for i in 1..=10 {
        client.publish("/uns/sample/topic/sensor/one", QoS::AtMostOnce, false, payload::payload_to_json_string(&payload::build_fake_data()).as_bytes()).unwrap();
        thread::sleep(Duration::from_secs(2));
    });
    

    // Note from docs -- // Iterate to poll the eventloop for connection progress
    for (i, notification) in connection.iter().enumerate() {}
}

fn main() {
    println!("Rust MQTT");
    mqtt()
}
