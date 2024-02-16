use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

use crud::process_request;
mod crud;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // create our client
    let client = reqwest::Client::new();

    loop {
        print!("> ");

        io::stdout().flush().unwrap();

        // get input and process it
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        process_request(input, &client).await;
    }

}
