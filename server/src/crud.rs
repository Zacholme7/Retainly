use actix_web::{get, post, put, web, HttpResponse};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

use crate::db::*;
use crate::logic::*;
use common::{Card, Outcome};

/// Insert a new card into the database
#[post("/insert_card")]
async fn insert_card(
    conn: web::Data<Arc<Mutex<Connection>>>,
    card: web::Json<Card>,
) -> HttpResponse {
    // acquire the connection
    let conn = conn.lock().unwrap();

    // insert the card into the database
    match insert_card_into_db(&conn, &card) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create the card"),
    }
}

#[get("/get_next_card")]
async fn get_next_card(state: web::Data<Arc<Mutex<SpacedRepetition>>>) -> HttpResponse {
    // this will call a function on the spaced repeition struct that will return the card that we
    let mut state = state.lock().unwrap();

    // If there is a card left in this review session, send it to the client.
    // Othwise, send back a not found meaning we do not have any others
    match state.get_next_card() {
        Some(card) => HttpResponse::Ok().json(card),
        None => HttpResponse::NotFound().finish(),
    }
}

#[put("/update_card/{outcome}:{id}")]
async fn update_card(
    conn: web::Data<Arc<Mutex<Connection>>>,
    path: web::Path<(String, i64)>,
) -> HttpResponse {
    // acquire the connection
    // extract the outcome and id from the path
    let (outcome, id) = path.into_inner();

    // convert outcome to Outcome structure
    let outcome = if outcome == "1" { Outcome::RIGHT } else { Outcome::WRONG };

    // acquire the connection
    let conn = conn.lock().unwrap();

    // get the card
    let card = match get_card(&conn, id) {
        Ok(card) => card,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    // update card level based on outcome
    let res = match outcome {
        Outcome::RIGHT => move_card_up_level(&conn, id),
        Outcome::WRONG => move_card_to_level_one(&conn, id)
    };


    // update the card in the spaced repeition
    // get access to the core
    //core.update_card(outcome, card);

    HttpResponse::Ok().finish()
}

/// Retrieve all of the cards in the database
#[get("/list_all_cards")]
async fn list_all_cards(conn: web::Data<Arc<Mutex<Connection>>>) -> HttpResponse {
    // acquire the connection
    let conn = conn.lock().unwrap();

    // get all of the cards and send it back to the client
    match query_cards(&conn) {
        Ok(cards) => HttpResponse::Ok().json(cards),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve all of the cards"),
    }
}
