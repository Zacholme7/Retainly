use actix_web::{web,post, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

mod crud;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_card)
            .service(update_card)
            .service(delete_card)
            .service(get_card)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
