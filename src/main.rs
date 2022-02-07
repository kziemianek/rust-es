mod es;

use crate::es::{EventSourced, SourceEvent};

impl EventSourced<Self> for Book {
    fn get_events(self) -> Vec<Box<dyn SourceEvent<Self>>> {
        self.events
    }

    fn apply(&mut self, ev: impl SourceEvent<Self>) {
        ev.apply(self)
    }
}

struct Book {
    author: String,
    pages: Vec<String>,
    events: Vec<Box<dyn SourceEvent<Book>>>,
}

pub struct BookCreated {
    author: String,
}

pub struct PageAdded {
    content: String,
}

impl SourceEvent<Book> for BookCreated {
    fn apply(&self, mut t: &mut Book) {
        t.author = self.author.to_owned();
    }
}

impl SourceEvent<Book> for PageAdded {
    fn apply(&self, t: &mut Book) {
        t.pages.push(self.content.to_owned());
    }
}

impl Book {
    pub fn from_events(events: Vec<Box<dyn SourceEvent<Book>>>) -> Book {
        let mut book = Book {
            author: "".to_owned(),
            pages: vec![],
            events: vec![],
        };

        events.iter().for_each(|ev| ev.apply(&mut book));
        book
    }

    pub fn new(author: String) -> Book {
        Book {
            author,
            pages: vec![],
            events: vec![],
        }
    }

    pub fn add_page(&mut self, page: String) {
        self.events.push(Box::new(PageAdded {
            content: page.to_owned(),
        }));
        self.pages.push(page);
    }
}

fn main() {
    let book_created = BookCreated {
        author: "ds".to_owned(),
    };
    let page_added = PageAdded {
        content: "first page".to_owned(),
    };
    let page_added_2 = PageAdded {
        content: "second page".to_owned(),
    };
    let events: Vec<Box<dyn SourceEvent<Book>>> = vec![
        Box::new(book_created),
        Box::new(page_added),
        Box::new(page_added_2),
    ];
    let book = Book::from_events(events);

    println!("{} {}", book.author, book.pages.len());
}

mod tests {
    use super::*;

    #[test]
    fn sources_from_events() {
        let book_created = BookCreated {
            author: "ds".to_owned(),
        };
        let page_added = PageAdded {
            content: "first page".to_owned(),
        };
        let page_added_2 = PageAdded {
            content: "second page".to_owned(),
        };
        let events: Vec<Box<dyn SourceEvent<Book>>> = vec![
            Box::new(book_created),
            Box::new(page_added),
            Box::new(page_added_2),
        ];
        let book = Book::from_events(events);
        assert_eq!(2, book.pages.len())
    }
    
}
