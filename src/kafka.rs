use kafka::producer::{AsBytes, Producer};

use crate::Book;

pub struct BookKafkaRepository {
    pub topic: String,
    producer: Producer,
}

impl BookKafkaRepository {
    pub fn new(brokers: Vec<String>) -> BookKafkaRepository {
        let producer = Producer::from_hosts(brokers).create().expect("msg");
        BookKafkaRepository {
            topic: "books".to_owned(),
            producer,
        }
    }
}

impl BookKafkaRepository {
    pub fn save(mut self, book: Book) {
        book.events.iter().for_each(|ev| {
            let bytes = serde_json::to_vec(ev)
                .expect("could not serialize")
                .as_bytes()
                .to_owned();
            self.producer
                .send(&kafka::producer::Record::from_value(&self.topic, bytes))
                .expect("msg")
        });
    }
}
