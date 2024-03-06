use common::{Card, URL};
use std::io::{self, Write};

/// Process a request from the cli
pub async fn process_request(
    input: String,
    client: &reqwest::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    match input.as_str().trim() {
        "add" => add_new_card(&client).await?, // add a new card to the deck
        "list" => list_cards(&client).await?,  // list of the cards currently in teh deck
        "delete" => delete_card(&client).await?, // delete a card from the deck
        "modify" => modify_card(&client).await?, // modify a card in the deck
        "start" => learn(&client).await?,      // start a learning session
        _ => return Err("Unknown command".into()),
    }
    Ok(())
}

/// Add a new card into the deck
async fn add_new_card(client: &reqwest::Client) -> Result<(), Box<dyn std::error::Error>> {
    // get the term and the definition
    println!(); // formatting
    print!("Enter the term: ");
    io::stdout().flush().unwrap();
    let term = read_trimmed_line();
    print!("Enter the definition: ");
    io::stdout().flush().unwrap();
    let definition = read_trimmed_line();
    println!(); // formatting

    // send a request to create a new card in the database
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
        println!(); // for formatting

        if cards.len() == 0 {
            println!("No cards have been added. Add some cards to get started");
        } else {
            for card in cards {
                println!(
                    "{}. Term: {}, Definition: {}, Current Level: {}",
                    card.id, card.term, card.definition, card.current_level
                );
            }
        }
        println!(); // for formatting
        read_trimmed_line(); // just so we can see the output before screen clea
        Ok(())
    } else {
        return Err(response.text().await?.into());
    }
}

/// Modify a card in the deck
async fn modify_card(client: &reqwest::Client) -> Result<(), Box<dyn std::error::Error>> {
    // list all of the cards so we know which ones we are able to modify
    list_cards(&client).await?;

    // Get the id of the card we want to modify, the new term, and the new definition
    print!("What is the id of the card you would like to modify: ");
    io::stdout().flush().unwrap();
    let id = read_trimmed_line();

    print!("What is the new term: ");
    io::stdout().flush().unwrap();
    let term = read_trimmed_line();

    print!("What is the new definition: ");
    io::stdout().flush().unwrap();
    let definition = read_trimmed_line();

    // send request to modify a card in the database
    let url = format!("{}/modify_card/{}:{}:{}", URL, id, term, definition);
    let response = client.put(url).send().await?;

    // process the response and handle errors
    if response.status().is_success() {
        return Ok(());
    } else {
        return Err(response.text().await?.into());
    }
}

/// Delete a card from the deck
pub async fn delete_card(client: &reqwest::Client) -> Result<(), Box<dyn std::error::Error>> {
    // list all of the cards so we know which ones we can delete
    list_cards(&client).await?;

    // Get the id of the card we want to delete
    print!("What is the id of the card you would like to delete: ");
    io::stdout().flush().unwrap();
    let id = read_trimmed_line();

    // send request to delete a card in the database
    let url = format!("{}/delete_card/{}", URL, id);
    let response = client.put(url).send().await?;

    // process the response and handle errors
    if response.status().is_success() {
        return Ok(());
    } else {
        return Err(response.text().await?.into());
    }
}

/// Check if the current day is in progress
async fn check_day_in_progress(client: &reqwest::Client) -> Result<bool, Box<dyn std::error::Error>>  {
        let url = format!("{}/is_day_in_progress", URL);
        let response = client.get(url).send().await?;

        // if the request is a success, the day is still in progress, else we have not started a day yet
        match response.status() {
                reqwest::StatusCode::OK => Ok(true),
                _ => Ok(false),
        }
}

/// Get the very last card that was shown to us
async fn get_last_card(client: &reqwest::Client) -> Result<Card, Box<dyn std::error::Error>> {
        let url = format!("{}/get_last_card", URL);
        let response = client.get(url).send().await?;

        match response.status() {
                reqwest::StatusCode::OK => {
                        let card = response.json::<Card>().await?;
                        Ok(card)
                }
                _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to fetch the next card",
                ))),
        }
}

/// Start the learning for the day
async fn learn(client: &reqwest::Client) -> Result<(), Box<dyn std::error::Error>> {
        // before we start learning, we need to check if the day was interrupted mid session last time
        let is_day_in_progress = check_day_in_progress(&client).await?;

        if is_day_in_progress {
                // if they day is in progress, we want to get the last card and present it
                let last_card = get_last_card(&client).await?;
                handle_card_review(&client, &last_card).await?;
        }

        // loop while we still have cards left in the day to learn
        loop {
                // get the card
                match get_next_card(client).await? {
                        // we have an other card left in the day
                        Some(card) => {
                                handle_card_review(&client, &card).await?;
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

async fn handle_card_review(client: &reqwest::Client, card: &Card) -> Result<(), Box<dyn std::error::Error>> {
        // print info
        println!();
        println!("Term: {}", card.term);
        print!("Outcome? show/y/n: ");

        // get outcome
        io::stdout().flush().unwrap();
        let mut outcome = read_trimmed_line();

        // if we want to see def, show it
        if outcome == "show" {
                println!("Definition: {}", card.definition);
                print!("Outcome? y/n: ");
                io::stdout().flush().unwrap();
                outcome = read_trimmed_line();
        }

        // update the card in the server with the outcome
        update_card(&client, card.clone(), outcome).await?;
        Ok(())
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

// Helper function to read user input
fn read_trimmed_line() -> String {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input.trim().to_string()
}
