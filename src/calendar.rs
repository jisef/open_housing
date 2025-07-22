use crate::booking::BookingParams;
use crate::data_objects::database::booking::Booking;
use crate::App;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use serde_json::json;
use std::sync::Arc;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;

// todo!("can be optimized")
pub async fn get_arrivals(
    State(app): State<Arc<App>>,
    Query(params): Query<CalendarParam>,
) -> impl IntoResponse {
    let mut sql = format!("SELECT * FROM view_arrival WHERE date::timestamp BETWEEN cast('{}'::timestamp as date) AND cast('{}'::timestamp as date)",params.start_date, params.end_date);
    let result = sqlx::query_as::<_, Arrival >(&sql)
        .fetch_all(&app.pool)
        .await;

    match result {
        Ok(x) => {
            let responses = json!({
                "status": "success",
                "data": x
            });
            axum::Json(responses)
        }
        Err(error) => {
            let respomse = json!({
                "status": "error",
                "error": format!("{}", error)
            });
            axum::Json(respomse)
        }
    }
}
#[derive(Deserialize, Serialize)]
pub struct Arrival {
    pub room_pk: i32,
    pub booking_pk: i32,
    pub date: NaiveDateTime,
    pub number: i32,
    pub name: String
}


impl<'r> FromRow<'r, PgRow> for Arrival{
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        Ok(Arrival{
            booking_pk: row.try_get("booking_pk")?,
            date: row.try_get("date")?,
            number: row.try_get("number")?,
            room_pk: row.try_get("room_pk")?,
            name: row.try_get("name")?,
        })
    }
}

#[derive(serde::Deserialize)]
pub struct CalendarParam {
    start_date: String,
    end_date: String,
}

pub async fn get_departures(
    State(app): State<Arc<App>>,
    Query(params): Query<CalendarParam>,
) -> impl IntoResponse {
}
