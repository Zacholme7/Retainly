use crud::{get_general_information, process_request};
use std::io::{self, Write};
mod crud;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create our client
    let client = reqwest::Client::new();

    loop {
        // Clear the screen
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();

        print_general_info(&client).await;
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

// prints general information about the state of the application
async fn print_general_info(client: &reqwest::Client) {
    let info = get_general_information(&client).await.unwrap();
    println!("------------------------------");
    println!("Day: {}, To Review: {:?}", info.day, info.levels);
    println!("------------------------------");
}
