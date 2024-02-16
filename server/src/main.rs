use actix_web::{web, App, HttpServer};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

use crud::*;
use sql::*;
mod crud;
mod sql;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // open up the database
    let conn = Connection::open("cards.db").unwrap();
    create_table(&conn).unwrap();

    let conn = Arc::new(Mutex::new(conn));

    // start the http server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conn.clone()))
            .service(insert_card)
            .service(list_all_cards)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
