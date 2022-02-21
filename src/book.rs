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
