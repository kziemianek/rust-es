mod book;
mod kafka;

use std::{thread, time};

use ::kafka::{
    client::{FetchOffset, GroupOffsetStorage},
    consumer::Consumer,
};

use crate::{
    book::{Book, BookEvent},
    kafka::BookKafkaRepository,
};

fn main() {
    let kafka_brokers = vec!["localhost:9092".to_owned()];
    let book_id = Book::new_id();
    let mut repository = BookKafkaRepository::new(kafka_brokers.clone());
    let mut book = get_book_or_create_new(&book_id, &repository);
    generate_new_book_page(&mut book, &mut repository);

    listen_to_books_topic(kafka_brokers);
}

fn get_book_or_create_new(id: &str, repository: &BookKafkaRepository) -> Book {
    if let Some(book) = repository.get(id) {
        book
    } else {
        Book::new(id.to_owned(), "Joe".to_owned())
    }
}

fn generate_new_book_page(book: &mut Book, repository: &mut BookKafkaRepository) {
    let next_page_number = book.pages.len() + 1;
    book.add_page(("Page #".to_owned() + &next_page_number.to_string()).to_owned());
    repository.save(book);
}

fn listen_to_books_topic(brokers: Vec<String>) {
    let mut con = Consumer::from_hosts(brokers.clone())
        .with_topic("books".to_owned())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .expect("Could not connect to kafka broker");

    loop {
        thread::sleep(time::Duration::from_millis(1000));
        let mss = con.poll().expect("msg");

        for ms in mss.iter() {
            for m in ms.messages() {
                let ev: BookEvent = serde_json::from_slice(m.value).expect("msg");

                println!("{}:{}@{}: {:?}", ms.topic(), ms.partition(), m.offset, ev);
            }
            let _ = con.consume_messageset(ms);
        }
        con.commit_consumed()
            .expect("Error while commititng consumed");
    }
}

#[cfg(test)]
mod tests {
    use crate::book::{Book, BookCreated, BookEvent, PageAdded};

    #[test]
    fn sources_from_events() {
        let book_created = BookEvent::Created(BookCreated {
            id: "1".to_owned(),
            author: "ds".to_owned(),
        });
        let page_added = BookEvent::PageAdded(PageAdded {
            content: "first page".to_owned(),
        });
        let page_added_2 = BookEvent::PageAdded(PageAdded {
            content: "second page".to_owned(),
        });
        let events: Vec<BookEvent> = vec![book_created, page_added, page_added_2];
        let book = Book::from_events(events);
        assert_eq!(2, book.pages.len())
    }
}
