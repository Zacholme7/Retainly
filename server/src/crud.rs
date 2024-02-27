use actix_web::{get, post, put, web, HttpResponse};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

use crate::db::*;
use crate::logic::*;
use common::{Card, Outcome, GeneralInfo};

/// Insert a new card into the database
#[post("/insert_card")]
async fn insert_card(
    conn: web::Data<Arc<Mutex<Connection>>>,
    state: web::Data<Arc<Mutex<SpacedRepetition>>>,
    card: web::Json<Card>,
) -> HttpResponse {
    // acquire the connection
    let conn = conn.lock().unwrap();

    // insert the card into the database
    let id = match insert_card_into_db(&conn, &card) {
        Ok(id) => id,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to create the card"),
    };

    // get the newly inserted card from the database
    let card = get_card(&conn, id).unwrap();

    // insert the card into the state
    let mut state = state.lock().unwrap();
    state.insert_card_into_level(card.clone());

    HttpResponse::Ok().finish()
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
    state: web::Data<Arc<Mutex<SpacedRepetition>>>,
    path: web::Path<(String, i64)>,
) -> HttpResponse {
    // acquire the connection
    // extract the outcome and id from the path
    let (outcome, id) = path.into_inner();

    // convert outcome to Outcome structure
    let outcome = if outcome == "1" {
        Outcome::RIGHT
    } else {
        Outcome::WRONG
    };

    // acquire the connection
    let conn = conn.lock().unwrap();

    // update the card level in the database
    if let Err(_) = move_card_level_in_db(&conn, &outcome, id) {
        return HttpResponse::InternalServerError().body("Failed to move card in db");
    }

    // get the newly updated card
    let card = get_card(&conn, id).unwrap();


    // update the card level in the state
    let mut state = state.lock().unwrap();
    if let Err(_) = state.move_card_level_in_state(card) {
        return HttpResponse::InternalServerError().body("Failed to move card in state levels");
    }

    // finished updating, return OK
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

/// Retrieves general information about the state of the application
#[get("/general_info")]
async fn get_general_info(state: web::Data<Arc<Mutex<SpacedRepetition>>>) -> HttpResponse {
    // acquire the state
    let state = state.lock().unwrap();

    // get the information
    let general_information = state.get_general_information();

    // send response
    HttpResponse::Ok().json(general_information)
}





































