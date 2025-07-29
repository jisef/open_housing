mod booking_handler;
mod common;
mod data_objects;
mod room_handler;

use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use booking_handler::add_booking;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::Deserialize;
use sqlx::{Executor, FromRow, Row};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{TraceLayer};

#[tokio::main]
async fn main() {
    let app = match App::new().await {
        Ok(app) => app,
        Err(e) => {
            eprintln!("Failed to start server: {}", e);
            return;
        }
    };

    let app_state = Arc::new(app);

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(
            Any, /*HeaderValue::from_static("http://localhost:5173")*/
        )
        .allow_headers(Any);

    let router = Router::new()
        .route("/api/bookings", get(booking_handler::get_bookings))
        .route("/api/bookings", post(add_booking))
        .route("/api/bookings/today", get(booking_handler::get_bookings_today))

        .route("/api/rooms", get(room_handler::get_rooms))
        .route("/api/rooms", post(room_handler::add_rooms))
        .route("/api/rooms/free", get(room_handler::get_room_is_free))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);

    if let Err(e) = axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), router).await {
        eprintln!("Server error: {}", e);
    }
}

struct App {
    connection: DatabaseConnection,
}

impl App {
    /// Creates Conn pool and applies schema
    async fn new() -> Result<App, sea_orm::error::DbErr> {
        let url = &*dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let mut options = ConnectOptions::new(url);
            options.min_connections(2)
            .sqlx_logging(true).set_schema_search_path("public");

        let conn = Database::connect(options).await?;

        Ok(App { connection: conn})
    }
}
