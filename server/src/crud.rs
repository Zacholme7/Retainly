
use actix_web::{web, post, HttpResponse};
use std::sync::{Arc, Mutex};
use rusqlite::Connection;

use common::Card;
use crate::sql::*;

/// Create a new card in the database
#[post("/create_new_card")]
async fn create_new_card(conn: web::Data<Arc<Mutex<Connection>>>, card: web::Json<Card>) -> HttpResponse {
    let conn = conn.lock().unwrap();
    let _ = insert_card(&conn, &card);


    HttpResponse::Ok().finish()
}

