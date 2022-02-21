use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub enum BookEvent {
    Created(BookCreated),
    PageAdded(PageAdded),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub id: String,
    pub author: String,
    pub pages: Vec<String>,
    pub events: Vec<BookEvent>,
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
    pub id: String,
    pub author: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PageAdded {
    pub content: String,
}

impl Book {
    pub fn new_id() -> String {
        Uuid::new(uuid::UuidVersion::Random)
            .expect("Could not generate UUID")
            .hyphenated()
            .to_string()
    }

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
            id: id.to_owned(),
            author: author.to_owned(),
            pages: vec![],
            events: vec![BookEvent::Created(BookCreated {
                id: id,
                author: author,
            })],
        }
    }

    pub fn add_page(&mut self, page: String) {
        self.events.push(BookEvent::PageAdded(PageAdded {
            content: page.to_owned(),
        }));
        self.pages.push(page);
    }

    pub fn clear_events(&mut self) {
        self.events.clear()
    }
}
