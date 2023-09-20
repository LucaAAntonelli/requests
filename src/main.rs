use chrono::{DateTime, Utc};
extern crate reqwest;

struct Book {
    title: String,
    author: String,
    release_date: DateTime<Utc>,
    pages: u16,
}

fn main() {
    println!("Hello, world!");
}

async fn search(query: String) -> Result<(), reqwest::Error> {
    let body = reqwest::get("https://www.googleapis.com/books/v1/volumes?q=".to_owned() + &query)
        .await?
        .text()
        .await?;
    println!("response = {body}");

    Ok(())
}
