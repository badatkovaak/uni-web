pub mod handlers;


use axum::{
    self,
    routing::get,
    Router,
};


#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" })).route("/echo", get(handlers::echo ));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::Server::bind()
    axum::serve(listener, app).await.unwrap();
}
