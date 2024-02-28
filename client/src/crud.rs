use common::{Card, GeneralInfo};
use std::io::{self, Write};

/// URL of the webserver
const URL: &str = "http://127.0.0.1:8080";

/// Process request
pub async fn process_request(
    input: String,
    client: &reqwest::Client,
) -> Result<(), Box<dyn std::error::Error>> {
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
        "start" => learn(&client).await?,
        _ => return Err("Unknown command".into()),
    }
    Ok(())
}

/// Start the learning for the day
async fn learn(client: &reqwest::Client) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // get the card
        match get_next_card(client).await? {
            // we have an other card left in the day
            Some(card) => {
                println!("Term: {}", card.term);
                // derermine how we answered it
                print!("Outcome? show/y/n: ");

                io::stdout().flush().unwrap();
                let mut outcome = read_trimmed_line();

                if outcome == "show" {
                    println!("Definition: {}", card.definition);
                    print!("Outcome? y/n: ");
                    io::stdout().flush().unwrap();
                    outcome = read_trimmed_line();
                } 

                // update the card in the server with the outcome
                update_card(&client, card, outcome).await?;
            }
            // there are no more cards left in the day
            None => {
                println!("day has ended");
                break;
            }
        }
    }
    Ok(())
}

/// Gets application information from the server
pub async fn get_general_information(client: &reqwest::Client) -> Result<GeneralInfo, Box<dyn std::error::Error>> {
    let url = format!("{}/general_info", URL);
    let response = client.get(url).send().await?;
    Ok(response.json::<GeneralInfo>().await?)
}

/// Gets the next card to learn
async fn get_next_card(
    client: &reqwest::Client,
) -> Result<Option<Card>, Box<dyn std::error::Error>> {
    let url = format!("{}/get_next_card", URL);
    let response = client.get(url).send().await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let card = response.json::<Card>().await?;
            Ok(Some(card))
        }
        reqwest::StatusCode::NOT_FOUND => Ok(None),
        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to fetch the next card",
        ))),
    }
}

/// Tells the server how the card should be updated
async fn update_card(
    client: &reqwest::Client,
    card: Card,
    outcome: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/update_card/{}:{}", URL, outcome, card.id);

    let response = client.put(url).send().await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to update the card",
        ))),
    }
}

/// Create a new card in the database
async fn create_card(
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
async fn list_cards(client: &reqwest::Client) -> Result<(), Box<dyn std::error::Error>> {
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


// Helper function to read user input
fn read_trimmed_line() -> String {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input.trim().to_string()
}
