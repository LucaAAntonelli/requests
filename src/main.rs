use lib::{json_to_books, search};
use std::io;

pub mod lib;

#[tokio::main]
async fn main() {
    let mut query = String::new();
    println!("Enter book to search");
    io::stdin()
        .read_line(&mut query)
        .expect("Failed to read line");
    let value = search(query).await.expect("Query returned an error");
    //  println!("The query returned the following result:\n{value}");
    let books = json_to_books(value);
    for book in books {
        println!("{book}");
    }
}
