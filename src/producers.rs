use log::info;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

mod logging;

async fn produce(server: &str, queue: &str) {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", server)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation failure");

    let futures = (0..5)
        .map(|i| async move {
            let delivery_status = producer
                .send(
                    FutureRecord::to(queue)
                        .payload(&format!("Message Chezburger{}", i))
                        .key(&format!("Key {}", i)), // .headers(OwnedHeaders::new().insert(Header {
                    //     key: "header_key",
                    //     value: Some("header_value")
                    // }))
                    Duration::from_secs(0),
                )
                .await;
            delivery_status
        })
        .collect::<Vec<_>>();

    for future in futures {
        info!("Future completed. Result: {:?}", future.await);
    }
}

#[tokio::main]
async fn main() {
    logging::setup_logger();
    info!("Hello world");
    let kafka = "localhost:9092";
    let queue = "incoming_orders";
    produce(kafka, queue).await;
}
