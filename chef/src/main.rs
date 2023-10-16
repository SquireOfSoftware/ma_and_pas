use std::{thread, time};
use std::time::Duration;
use log::{error, info, warn};
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::TopicPartitionList;
use rnglib::{Language, RNG};
use serde::Deserialize;

mod logging;

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance<'a>(&self, rebalance: &Rebalance<'a>) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance<'a>(&self, rebalance: &Rebalance<'a>) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

type LoggingConsumer = StreamConsumer<CustomContext>;

#[derive(Deserialize, Debug)]
enum DishType {
    burger,
    fries,
    drink
}

fn get_cook_time_in_millis(dish_type: DishType) -> u64 {
    match dish_type {
        DishType::burger => 54, // 9 mins - but shorted to 54 millis
        DishType::fries => 30, // 5 mins - but shorted to 30 millis
        DishType::drink => 18, // 3 mins - but shorted to 18 millis
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename(deserialize = "camelCase"))]
struct CookRequest {
    subOrderId: String,
    dishType: DishType,
    dishName: String,
    orderId: String,
}

async fn cooking_loop(server: &str, consumption_queue: &str, acknowledgement_queue: &str) {
    let context = CustomContext;
    let consumer: LoggingConsumer = ClientConfig::new()
        .set("bootstrap.servers", server)
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("group.id", "incomingOrders")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", server)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation failure");

    let topics: [&str; 1] = [&consumption_queue];

    consumer.subscribe(&topics)
        .expect("Can't subscribe to topics");

    loop {
        match consumer.recv().await {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        warn!("Error while deserialising message payload: {:?}", e);
                        ""
                    }
                };
                info!("Tryna read {}", payload);
                let cook_request: CookRequest = serde_json::from_str(payload).expect("Test");

                info!("key: '{:?}', cook_request: '{:?}', topic: '{}', partition: '{}', offset: '{}', timestamp: '{:?}'",
                    m.key(), cook_request, m.topic(), m.partition(), m.offset(), m.timestamp());
                info!("Received sub order: {:?}", cook_request);
                // consumer.commit_message(&m, CommitMode::Async).unwrap();

                info!("Cooking the sub order now...");
                // need to parse the json that is sent down
                // need to emit a cooking event
                let cook_time = get_cook_time_in_millis(cook_request.dishType);

                thread::sleep(Duration::from_millis(cook_time));

                info!("{} is finished, sending the acknowledgement!", payload);

                complete_order(producer, acknowledgement_queue, payload).await;

                info!("Acknowledgement for {} is sent? Please check that it was received", payload);
            }
        }
    }
}

async fn complete_order(producer: &FutureProducer, queue: &str, order_id: &str) {
    producer
        .send(
            FutureRecord::to(queue)
                .payload(&format!("{}", order_id))
                .key(&format!("Key {}", order_id))
            ,
            Duration::from_secs(0)
        )
        .await
        .expect("Must send acknowledgement");
}

#[tokio::main]
async fn main() {
    logging::setup_logger();
    let rng = RNG::try_from(&Language::Fantasy).unwrap();

    let first_name = rng.generate_name();

    info!("{}", format!("This is chef: {} at your service", first_name));
    let kafka = "localhost:19092";
    let consumption_queue = "incoming_sub_orders";
    let acknowledgement_queue = "acknowledgements";
    cooking_loop(kafka, consumption_queue, acknowledgement_queue).await;
}