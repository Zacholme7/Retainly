use reqwest::Error;
use common::Card;

/// URL of the webserver
const URL: &str = "http://127.0.0.1:8080";


/// Process request from tui to update card state
pub fn process_request(input: String) {
    println!("{input}");
}

/// Request to create a new card
pub async fn create_card(client: &reqwest::Client, term: &str, definition: &str) -> Result<(), Error> {

    // create the new card and construct the url
    let url = format!("{}/create_new_card", URL);
    let new_card = Card::new(term, definition);

    // send the request
    let response = client
        .post(url)
        .json(&new_card)
        .send()
        .await?;
        //.text()
        //.await;


    println!("{:?}", response);
    Ok(())
}


/// Request to update a card
pub async fn update_card() {
    let url = format!("{}/update_card", URL);
    todo!()
}

/// Request to delete a card
pub async fn delete_card() {
    todo!()
}


/// Request to get a card
pub async fn get_card() {
    todo!()
}
