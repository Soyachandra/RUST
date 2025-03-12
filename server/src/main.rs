use std::net::SocketAddr;
use axum::{Router, routing::{get, post, put, delete}, Json};
use tokio::net::TcpListener; 

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/create", post(|| async { "created" }))
        .route("/update", put(|| async { "updated" }))
        .route("/delete", delete(|| async { "deleted" }));

        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        println!("Server running at http://{}", addr);
    
        let listener = TcpListener::bind(addr).await.unwrap(); 
        axum::serve(listener, app.into_make_service()) 
        .await
        .unwrap();
}


// use std::net::SocketAddr;
// use axum::{Router, routing::{get, post, put, delete}};
// use hyper::server::Server;

// #[tokio::main]
// async fn main() {
//     // Define routes
//     let app = Router::new()
//         .route("/", get(|| async { "Hello World" }))
//         .route("/create", post(|| async { "created" }))
//         .route("/update", put(|| async { "updated" }))
//         .route("/delete", delete(|| async { "deleted" }));

//     // Define server address
//     let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
//     println!("Server running at http://{}", addr);

//     // Use Hyper's Server instead of TcpListener
//     Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }
