use kafka::producer::{Producer, Record, RequiredAcks};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let topic = "learning-rust-topic";
    let host = "localhost:29092";

    let mut producer = Producer::from_hosts(vec![host.to_string()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    for x in 0..10 {
        let message = format!("message [{}]", x);

        producer
            .send(&Record::from_value(topic, message.as_bytes()))
            .unwrap();
    }

    Ok(())
}