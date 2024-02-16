use common::Card;
use std::io::{self, Write};

/// URL of the webserver
const URL: &str = "http://127.0.0.1:8080";

/// Process request
pub async fn process_request(
    input: String,
    client: &reqwest::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", input);
    match input.as_str().trim() {
        "add" => {
            print!("Enter the term: ");
            io::stdout().flush().unwrap();
            let term = read_trimmed_line();

            print!("Enter the definition: ");
            io::stdout().flush().unwrap();
            let definition = read_trimmed_line();

            create_card(&client, term, definition).await?;
        }
        "list" => list_cards(&client).await?,
        _ => return Err("Unknown command".into()),
    }
    Ok(())
}

/// Create a new card in the database
pub async fn create_card(
    client: &reqwest::Client,
    term: String,
    definition: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // create url, card, and send request
    let url = format!("{}/insert_card", URL);
    let new_card = Card::new(term, definition);
    let response = client.post(url).json(&new_card).send().await?;

    // process the response and handle errors
    if response.status().is_success() {
        return Ok(());
    } else {
        return Err(response.text().await?.into());
    }
}

/// Get all of the cards in the database
pub async fn list_cards(client: &reqwest::Client) -> Result<(), Box<dyn std::error::Error>> {
    // construct the url and send the request
    let url = format!("{}/list_all_cards", URL); // Ensure the endpoint matches the server's route
    let response = client.get(&url).send().await?;

    // process the response and handle errors
    if response.status().is_success() {
        let cards: Vec<Card> = response.json().await?;
        println!("Cards: {:?}", cards);
        Ok(())
    } else {
        return Err(response.text().await?.into());
    }
}

fn read_trimmed_line() -> String {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input.trim().to_string()
}
