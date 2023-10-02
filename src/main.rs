extern crate reqwest;
use serde::Deserialize;
use serde_json::Value;
use std::io;

#[derive(Debug, Deserialize)]
struct Book {
    title: String,
    author: Vec<String>,
    pages: u16,
}
#[tokio::main]
async fn main() {
    let mut query = String::new();
    println!("Enter book to search");
    io::stdin()
        .read_line(&mut query)
        .expect("Failed to read line");
    println!("Calling search function with query={query}");
    let value = search(query).await.expect("Query returned an error");
    //  println!("The query returned the following result:\n{value}");
    println!("Trying to call json_to_book on result");
    let book = json_to_book(value);
    //println!("Book: {book:?}");
}

async fn search(query: String) -> Result<String, reqwest::Error> {
    let body = reqwest::get("https://www.googleapis.com/books/v1/volumes?q=".to_owned() + &query)
        .await?
        .text()
        .await?;

    Ok(body)
}

fn json_to_book(json: String) -> Book {
    let book = Book {
        title: String::from("asdf"),
        author: vec![String::from("asdf")],
        pages: 0,
    };
    let collection: Value = serde_json::from_str(&json).expect("Failed to parse JSON");
    let items = &collection["items"].as_array();
    for item in items.unwrap() {
        let title = &item["volumeInfo"]["title"];
        let authors = &item["volumeInfo"]["authors"];
        println!("{title}, {authors}");
    }

    book
}
