mod auth_handler;
mod booking_handler;
mod common;
mod data_objects;
mod room_handler;
pub mod templates;
mod user_handler;

use crate::auth_handler::Backend;
use crate::common::db;
use axum::response::IntoResponse;
use axum::routing::{delete, get, patch, post};
use axum::Router;
use axum_login::{login_required, AuthManagerLayerBuilder};
use booking_handler::add_booking;
use migration::{Migrator, MigratorTrait};
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection,
    TransactionTrait,
};
use sea_query::SchemaStatementBuilder;
use serde::Deserialize;
use sqlx::{Executor, FromRow, Row};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_sessions::cookie::time::Duration;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_seaorm_store::PostgresStore;

const DEFAULT_LIMIT: u64 = 100u64;

#[tokio::main]
async fn main() {
    let app = match App::new().await {
        Ok(app) => app,
        Err(e) => {
            eprintln!("Failed to start server: {}", e);
            return;
        }
    };

    app.serve().await;
}
#[derive(Clone)]
struct App {
    connection: DatabaseConnection,
}

impl App {
    /// Creates Conn pool and applies schema
    async fn new() -> Result<App, sea_orm::error::DbErr> {
        let conn = db::connect(Some(true)).await;

        Ok(App { connection: conn })
    }

    async fn serve(self) {
        let session_store = PostgresStore::new(self.connection.clone());
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false) //TODO: CHANGE IF ITS PROD
            .with_expiry(Expiry::OnInactivity(Duration::minutes(7)));

        let backend = Backend::create().await;
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        let cors = CorsLayer::new()
            .allow_methods(Any)
            .allow_origin(Any)
            .allow_headers(Any);

        let router = Router::new()
            .route("/api/bookings", get(booking_handler::get_bookings))
            .route("/api/bookings", post(add_booking))
            .route(
                "/api/bookings/{booking_pk}",
                get(booking_handler::get_booking),
            )
            .route(
                "/api/bookings/{booking_pk}",
                delete(booking_handler::delete_booking),
            )
            .route(
                "/api/bookings/{booking_pk}",
                patch(booking_handler::patch_booking),
            )
            .route(
                "/api/bookings/today",
                get(booking_handler::get_bookings_today),
            )
            .route("/api/rooms", get(room_handler::get_rooms))
            .route("/api/rooms", post(room_handler::add_room))
            .route("/api/rooms/free", get(room_handler::get_room_is_free))
            .route("/api/rooms/{room_pk}", get(room_handler::get_room))
            .route("/api/rooms/{room_pk}", patch(room_handler::update_room))
            .route("/api/rooms/{room_pk}", delete(room_handler::delete_room))

            .route("/api/user", post(user_handler::add_user))

            .route_layer(login_required!(Backend, login_url = "/api/login"))
            .route("/api/login", post(auth_handler::login))
            .layer(cors)
            .layer(auth_layer)
            .with_state(Arc::new(self));

        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
        println!("Server listening on {}", addr);

        if let Err(e) =
            axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), router).await
        {
            eprintln!("Server error: {}", e);
        }
    }
}
