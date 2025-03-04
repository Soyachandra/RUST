use std::net::SocketAddr;
use axum::{Router,routing::{get,post},Json};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
                            .route("/",get(|| async {"Hello World"}))
                            .route("/create",post(create));
                            
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Server running at {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
#[derive(Debug, Deserialize, Serialize)]
struct Data {
    value: String,
}

async fn create(Json(payload): Json<Data>) -> Json<Data> {
    println!("Received: {:?}", payload);
    Json(payload)
}