use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct RequestData {
    name: String,
}

#[derive(Serialize)]
struct ResponseData {
    message: String,
}


async fn greet_user(info: web::Json<RequestData>) -> HttpResponse {
    let response = ResponseData {
        message: "hello world".to_string()
    };
    HttpResponse::Ok().json(response)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/greet", web::post().to(greet_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
