use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

use crud::process_request;
mod crud;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();

    loop {
        print!("> ");

        io::stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => process_request(input),
            Err(_) => println!("There was an error getting input"),
        }
    }

}
