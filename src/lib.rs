extern crate reqwest;
use serde::Deserialize;
use serde_json::Value;
#[derive(Deserialize)]

pub struct Book {
    title: String,
    author: Vec<String>,
    pages: u64,
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Title: {}, Author: {}, Pages: {}",
            self.title,
            self.author.join(", "),
            self.pages
        )
    }
}

pub async fn search(query: String) -> Result<String, reqwest::Error> {
    let body = reqwest::get("https://www.googleapis.com/books/v1/volumes?q=".to_owned() + &query)
        .await?
        .text()
        .await?;

    println!("{body}");
    Ok(body)
}

fn json_to_book(json: &Value) -> Book {
    let title = json["volumeInfo"]["title"]
        .as_str()
        .expect("Could not convert title to string")
        .to_owned();
    let authors_val = json["volumeInfo"]["authors"]
        .as_array()
        .expect("Could not convert authors to vector of strings");
    let authors = authors_val
        .iter()
        .map(|v| v.to_string().to_owned())
        .collect();
    let pages = json["volumeInfo"]["pageCount"].as_u64().unwrap_or_default();
    Book {
        title,
        author: authors,
        pages,
    }
}

pub fn json_to_books(json: String) -> Vec<Book> {
    let mut books: Vec<Book> = vec![];
    let collection: Value = serde_json::from_str(&json).expect("Failed to parse JSON");
    let items = &collection["items"].as_array();
    for item in items.expect("Failed to iterate over JSON objects") {
        let book = json_to_book(item);
        books.push(book);
    }

    books
}