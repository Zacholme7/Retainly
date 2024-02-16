use actix_web::{web,post, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conn.clone()))
            .service(create_new_card)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
