use std::io::{self, Write};
use crud::process_request;
mod crud;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create our client
    let client = reqwest::Client::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        // get input
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);

        // process request and handle error
        match process_request(input, &client).await {
            Ok(_) => (),
            Err(e) => println!("Error occured: {}", e),
        }
    }
}
