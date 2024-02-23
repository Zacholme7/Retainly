use actix_web::{web, put, get, post, HttpResponse};
use std::sync::{Arc, Mutex};
use rusqlite::Connection;


use common::Card;
use crate::db::*;
use crate::logic::*;

/// Insert a new card into the database
#[post("/insert_card")]
async fn insert_card(conn: web::Data<Arc<Mutex<Connection>>>, card: web::Json<Card>) -> HttpResponse {
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
        None => HttpResponse::NotFound().finish()
    }
}

#[put("/update_card/{outcome}")]
async fn update_card(conn: web::Data<Arc<Mutex<Connection>>>) -> HttpResponse {
    todo!()
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

