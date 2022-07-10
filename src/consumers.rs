use log::{info, warn};
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::Message;
use rdkafka::TopicPartitionList;

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

async fn consume(server: &str, topic: &str) {
    let context = CustomContext;
    let consumer: LoggingConsumer = ClientConfig::new()
        .set("bootstrap.servers", server)
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("group.id", "test")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    let topics: [&str; 1] = [&topic];

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
                info!("key: '{:?}', payload: '{}', topic: '{}', partition: '{}', offset: '{}', timestamp: '{:?}'",
                    m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        }
    }
}

#[tokio::main]
async fn main() {
    logging::setup_logger();
    info!("Hello world");
    let kafka = "localhost:9092";
    let topic = "incoming_orders";
    consume(kafka, topic).await;
}