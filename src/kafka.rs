use kafka::producer::{AsBytes, Producer, Record};
use rocksdb::{DBWithThreadMode, SingleThreaded, DB};

use crate::book::Book;

pub struct BookKafkaRepository {
    pub topic: String,
    producer: Producer,
    db: DBWithThreadMode<SingleThreaded>,
}

impl BookKafkaRepository {
    pub fn new(brokers: Vec<String>) -> BookKafkaRepository {
        let producer = Producer::from_hosts(brokers).create().expect("msg");
        let path = "./db/";
        let db = DB::open_default(path).unwrap();

        BookKafkaRepository {
            topic: "books".to_owned(),
            producer,
            db,
        }
    }
}

impl BookKafkaRepository {
    pub fn save(&mut self, book: &mut Book) {
        let mut records: Vec<Record<(), Vec<u8>>> = vec![];
        for ev in &book.events {
            let bytes = serde_json::to_vec(&ev)
                .expect("could not serialize")
                .as_bytes()
                .to_owned();
            records.push(kafka::producer::Record::from_value(&self.topic, bytes));
        }
        self.producer.send_all(&records).expect("msg");
        book.clear_events();
        let seriaized_events = serde_json::to_vec(&book).expect("msg");
        //save book as json, not events
        self.db
            .put(book.id.as_bytes(), seriaized_events)
            .expect("msg");
    }

    pub fn get(&self, id: &str) -> Option<Book> {
        match self.db.get(id.as_bytes()) {
            Ok(raw_book) => {
                if let Some(raw_book) = raw_book {
                    serde_json::from_slice(&raw_book).expect("err msg")
                } else {
                    None
                }
            }
            Err(e) => None,
        }
    }
}
