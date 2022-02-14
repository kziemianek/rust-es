mod kafka;

use std::{thread, time};

use ::kafka::{
    client::{FetchOffset, GroupOffsetStorage},
    consumer::Consumer,
};
use serde::{Deserialize, Serialize};

use crate::kafka::BookKafkaRepository;

#[derive(Serialize, Deserialize, Debug)]
pub enum BookEvent {
    Created(BookCreated),
    PageAdded(PageAdded),
}

pub struct Book {
    id: String,
    author: String,
    pages: Vec<String>,
    events: Vec<BookEvent>,
}

impl BookEvent {
    pub fn apply(&self, book: &mut Book) {
        match self {
            BookEvent::Created(ev) => {
                book.id = ev.id.to_owned();
                book.author = ev.author.to_owned();
            }
            BookEvent::PageAdded(ev) => {
                book.pages.push(ev.content.to_owned());
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookCreated {
    id: String,
    author: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PageAdded {
    content: String,
}

impl Book {
    pub fn from_events(events: Vec<BookEvent>) -> Book {
        let mut book = Book {
            id: String::new(),
            author: "".to_owned(),
            pages: vec![],
            events: vec![],
        };

        events.iter().for_each(|ev| ev.apply(&mut book));
        book
    }

    pub fn new(id: String, author: String) -> Book {
        Book {
            id,
            author,
            pages: vec![],
            events: vec![],
        }
    }

    pub fn add_page(&mut self, page: String) {
        self.events.push(BookEvent::PageAdded(PageAdded {
            content: page.to_owned(),
        }));
        self.pages.push(page);
    }
}

fn main() {
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
    let mut book = Book::from_events(events);

    book.add_page("third page".to_owned());

    let repository = BookKafkaRepository::new(vec!["localhost:9092".to_owned()]);
    repository.save(book);

    let mut con = Consumer::from_hosts(vec!["localhost:9092".to_owned()])
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
    use crate::{Book, BookCreated, BookEvent, PageAdded};

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
