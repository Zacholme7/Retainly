use actix_web::{web, App, HttpServer};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

use crud::*;
use db::*;
use logic::*;
mod crud;
mod db;
mod logic;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // open up the database
    let conn = Connection::open("cards.db").unwrap();
    create_table(&conn).unwrap();

    // database connection to be passed around
    let conn = Arc::new(Mutex::new(conn));

    // create the core of the application
    let core = Arc::new(Mutex::new(SpacedRepetition::new()));

    // update core to contain the state of the database
    core.lock().unwrap().initial_level_update(&conn.lock().unwrap());

    // start the http server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conn.clone()))
            .app_data(web::Data::new(core.clone()))
            .service(insert_card)
            .service(list_all_cards)
            .service(get_next_card)
            .service(update_card)
            .service(get_general_info)
            .service(modify_card)
            .service(delete_card)
            .service(is_day_in_progress)
            .service(get_last_card)
    })
    .bind("0.0.0.1:8080")?
    .run()
    .await
}
