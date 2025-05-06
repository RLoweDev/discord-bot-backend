use axum::Router;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing_subscriber;

mod db;
mod redis;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .nest("/api", routes::clock::routes())
        .nest("/api", routes::auth::routes())
        .nest("/api", routes::logs::routes())
        .nest("/api", routes::report::routes());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Server running at http://127.0.0.1:3000");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
