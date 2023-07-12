use kafka::consumer::{Consumer, FetchOffset};

fn main() {
    let topic = "learning-rust-topic";
    let host = "localhost:29092";

    let mut consumer =
        Consumer::from_hosts(vec!(host.to_string()))
            .with_topic(topic.to_string())
            .with_fallback_offset(FetchOffset::Earliest)
            .create()
            .unwrap();
    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let str = String::from_utf8_lossy(m.value);
                println!("{:?}", str);
            }
            let _ = consumer.consume_messageset(ms);
        }
        consumer.commit_consumed().unwrap();
    }
}