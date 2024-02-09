
use actix_web::{post, HttpResponse};
use common::Card;

/// Create a new card in the database
#[post("/create_new_card")]
async fn create_new_card(info: web::Json<Card>) -> HttpResponse {
}

/// Update the card in the database
async fn update_card() {
    todo!()
}

/// Delete the card from the database
async fn delete_card() {
    todo!()
}

/// Get a card from the database
async fn get_card() {
    todo!()
}
