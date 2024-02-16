use common::Card;
use reqwest::Error;
use std::io::{self, Write};

/// URL of the webserver
const URL: &str = "http://127.0.0.1:8080";

/// Process request from tui to update card state
pub async fn process_request(input: String, client: &reqwest::Client) {
    println!("{}", input);
    match input.as_str().trim() {
        "add" => {
            print!("Enter the term: ");
            io::stdout().flush().unwrap();
            let term = read_trimmed_line();

            print!("Enter the definition: ");
            io::stdout().flush().unwrap();
            let definition = read_trimmed_line();

            println!("Term {}, definition {}", term, definition);
            create_card(&client, term, definition).await.unwrap();
        },
        "list" => {
            list_cards(&client).await;

        }

        _ => println!("unknown command"),
    }
}

/// Request to create a new card
pub async fn create_card(
    client: &reqwest::Client,
    term: String,
    definition: String,
) -> Result<(), Error> {
    // create the new card and construct the url
    let url = format!("{}/create_new_card", URL);
    let new_card = Card::new(term, definition);

    // send the request
    let response = client.post(url).json(&new_card).send().await?;
    //.text()
    //.await;

    println!("{:?}", response);
    Ok(())
}

pub async fn list_cards(client: &reqwest::Client) {
    println!("running");
    let url = format!("{}/list_all", URL);
    let response = client.post(url).send().await;
}

fn read_trimmed_line() -> String {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input.trim().to_string()
}
