use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Router,routing::{get, post, put, delete}, extract::State, Json};
use tokio::net::TcpListener;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use serde::{Deserialize, Serialize};
//use hyper::Server;  

#[tokio::main]
async fn main() {
    let client = Client::with_options(ClientOptions::parse("mongodb://localhost:27017").await.unwrap()).unwrap();
    let db = Arc::new(client.database("testdb"));

    let app = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/create", post(create_user))
        .route("/update", put(update_user))
        .route("/delete", delete(delete_user))
        .with_state(db);

        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        let listener = TcpListener::bind(addr).await.unwrap();
        println!("Server running at {}", addr);
    

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize, Serialize,Clone)]
struct Data {
    id: u32,
    name: String,
    age: u32,
}

async fn create_user(State(db): State<Arc<Database>>, Json(payload): Json<Data>) -> Json<Data> {
    let collection = db.collection::<Data>("users");
    collection.insert_one(payload.clone(), None).await.unwrap();
    println!("User Created: {:?}", payload);
    Json(payload)
}

async fn update_user() -> &'static str {
    "User updated"
}

async fn delete_user() -> &'static str {
    "User deleted"
}
