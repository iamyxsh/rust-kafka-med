use rdkafka::{
    consumer::{BaseConsumer, Consumer},
    ClientConfig, Message,
};

pub fn create_consumer(brokers: &str, group_id: &str) -> BaseConsumer {
    ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .create()
        .unwrap()
}

pub async fn sub_to_topic(brokers: &str, group_id: &str, topics: &[&str]) {
    let consumer = create_consumer(brokers, group_id);

    consumer
        .subscribe(&topics.to_vec())
        .expect("failed to subscribe");

    for msg in consumer.iter() {
        match msg {
            Err(e) => println!("error recieving msg: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        println!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                println!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
            }
        }
    }
}
