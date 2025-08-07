mod booking_handler;
mod common;
mod data_objects;
mod room_handler;
pub mod templates;

use axum::response::IntoResponse;
use axum::routing::{delete, get, patch, post};
use axum::Router;
use booking_handler::add_booking;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Schema, TransactionTrait};
use serde::Deserialize;
use sqlx::{Executor, FromRow, Row};
use std::net::SocketAddr;
use std::sync::Arc;
use migration::{Migrator, MigratorTrait};
use sea_query::SchemaStatementBuilder;
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
        .route("/api/bookings/{booking_pk}", get(booking_handler::get_booking))
        .route("/api/bookings/{booking_pk}", delete(booking_handler::delete_booking))
        .route("/api/bookings/{booking_pk}", patch(booking_handler::patch_booking))
        .route("/api/bookings/today", get(booking_handler::get_bookings_today))

        .route("/api/rooms", get(room_handler::get_rooms))
        .route("/api/rooms", post(room_handler::add_room))
        .route("/api/rooms/free", get(room_handler::get_room_is_free))
        .route("/api/rooms/{room_pk}", get(room_handler::get_room))
        .route("/api/rooms/{room_pk}", patch(room_handler::update_room))
        .route("/api/rooms/{room_pk}", delete(room_handler::delete_room))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
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
        let url: String = match dotenvy::var("DATABASE_URL") {
            Ok(u) => {u}
            Err(_) => {std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")}
        };
        let url = &*url;
        let schema = dotenvy::var("SCHEMA").unwrap_or("public".to_string());
        let mut options = ConnectOptions::new(url);
            options.min_connections(2)
            .sqlx_logging(true).set_schema_search_path("public");
        println!("Connecting to db: {}", url);
        let conn = Database::connect(options).await?;
        println!("Connected to db: {}", url);
        conn.execute_unprepared(format!("SET search_path TO {schema}").as_str()).await?;
        Migrator::up(&conn, None).await?;
        println!("Migration applied");

        Ok(App { connection: conn})
    }
}


