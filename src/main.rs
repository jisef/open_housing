mod common;
mod data_objects;

use crate::data_objects::booking::{Booking, BookingData};
use axum::routing::get;
use axum::{
    extract::{Query, State},
    response::Json,
    Router,
};
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::json;
use sqlx::{Executor, Pool, Postgres, Row};
use std::net::SocketAddr;
use std::sync::Arc;
use axum::response::IntoResponse;

#[tokio::main]
async fn main() {
    // Database URL
    let database_url = &*dotenvy::var("DATABASE_URL").expect("DATABASE_URL not set");

    // Run migrations
    println!("Running database migrations...");

    let app = match App::new().await {
        Ok(app) => app,
        Err(e) => {
            eprintln!("Failed to start server: {}", e);
            return;
        }
    };

    let app_state = Arc::new(app);

    let router = Router::new()
        .route("/bookings", get(get_bookings))
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

        sqlx::query(&*BookingData::get_create_statement())
            .execute(&pool)
            .await?;

        Ok(App { pool })
    }
}
#[derive(Deserialize)]
pub struct BookingParams {
    pub id: Option<i32>,
    pub limit: Option<i32>,
    pub date: Option<NaiveDate>,
}
async fn get_bookings(
    State(app): State<Arc<App>>,
    Query(params): Query<BookingParams>,
) -> impl IntoResponse {
    // Example of using the connection pool to query the database
    let mut query_builder = SeaQuery::select().from(Booking::Table).to_owned();
    let mut limit = 0 as u64;
    if let Some(l) = params.limit {
        limit = l as u64;
    }

    let query = query_builder.build(PostgresQueryBuilder::default());
    let result = sqlx::query_as::<_, BookingData>(&*query.0)
        .fetch_one(&app.pool)
        .await;

    match result {
        Ok(row) => {
            // Extract the message from the row
            let response = json!({
                "status": "success",
                "message": row.id.to_string(),
                "limit": params.limit,
            });

            Json(response)
        }
        Err(e) => {
            eprintln!("Database error: {}", e);

            // Return an error response
            let error_response = json!({
                "status": "error",
                "message": format!("Database error: {}", e)
            });

            Json(error_response)
        }
    }
}
