mod common;
mod data_objects;
mod booking;
mod overview;
mod room;
mod calendar;

use axum::routing::{get, post};
use axum::{  Router};
use serde::Deserialize;
use sqlx::{Executor, FromRow, Pool, Postgres, Row};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use axum::response::IntoResponse;
use tower_http::cors::{Any, CorsLayer};
use booking::add_booking;
use crate::overview::get_overview;

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

    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any/*HeaderValue::from_static("http://localhost:5173")*/).allow_headers(Any);
    let cors = CorsLayer::very_permissive();

    let router = Router::new()
        //.route("/booking/{id}", get(get_booking))
        .route("/bookings", get(booking::get_bookings))
        .route("/overview", get(get_overview))
        .route("/api/add_booking", post(add_booking))
        .route("/arrivals", get(calendar::get_arrivals))
        .route("/departures", get(calendar::get_departures))
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
    pool: Pool<Postgres>,
}

impl App {
    /// Creates Conn pool and applies schema
    async fn new() -> Result<App, sqlx::Error> {
        let pool = common::db::create_conn_pool().await?;

        Ok(App { pool })
    }
}


