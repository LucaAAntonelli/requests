use chrono::{DateTime, Utc};
extern crate reqwest;
use std::io;

struct Book {
    title: String,
    author: String,
    release_date: DateTime<Utc>,
    pages: u16,
}
#[tokio::main]
async fn main() {
    let mut query = String::new();
    println!("Enter book to search");
    io::stdin().read_line(&mut query).expect("Failed to read line");
    println!("Calling search function with query={query}");
    let value = search(query).await.expect("Query returned an error");
    println!("The query returned the following result:\n{value}");
}

async fn search(query: String) -> Result<String, reqwest::Error> {
    let body = reqwest::get("https://www.googleapis.com/books/v1/volumes?q=".to_owned() + &query)
        .await?
        .text()
        .await?;

    Ok(body)
}
