use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct RequestData {
    name: String,
}

#[derive(Deserialize)]
struct ResponseData {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let url = "http://127.0.0.1:8080/greet";

    // Data to send
    let request_data = RequestData {
        name: "Alice".into(),
    };

    // Send a POST request and wait for the JSON response
    let response = client
        .post(url)
        .json(&request_data)
        .send()
        .await?
        .json::<ResponseData>()
        .await?;

    println!("Server response: {}", response.message);

    Ok(())
}
