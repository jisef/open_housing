use std::str::FromStr;
use std::sync::Arc;
use axum::extract::{Query, State};
use axum::{Form, Json};
use axum::response::IntoResponse;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{App};
use crate::common::datetime::DateTime;
use crate::data_objects::database::booking::Booking;

pub async fn get_bookings(
    State(app): State<Arc<App>>,
    Query(params): Query<BookingParams>,
) -> impl IntoResponse {

    let mut sql = String::from("SELECT * FROM booking WHERE valid = true");
    if let Some(limit) = params.limit {
        sql = format!("SELECT  * FROM booking WHERE valid = true LIMIT {limit}")
    }

    let result = sqlx::query_as::<_, Booking>(&sql).fetch_all(&app.pool).await;

    match result {
        Ok(bookings) => {
            let response = json!({
                "status": "success",
                "amount": bookings.len(),
                "message": bookings,
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
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BookingParams {
    pub id: Option<i32>,
    pub limit: Option<i32>,
    pub date: Option<NaiveDate>,
}


pub async fn add_booking(
    State(app): State<Arc<App>>,
    Form(booking): Form<AddBookingForm >,
) -> impl IntoResponse {
    //check params
    booking.check();

    // into db
    let mut result = sqlx::query(
        "INSERT INTO booking(date_start, date_end, room_fk)
        VALUES ($1::timestamp, $2::timestamp, $3)")
        .bind(booking.date_start)
        .bind(booking.date_end)
        .bind(booking.room_pk)
        .execute(&app.pool).await;

    match result {
        Ok(x) => {
            let x = x.rows_affected();
            let resp = json!({
                "status": "success",
                "affected_rows": x
            });
            Json(resp)
        }
        Err(x) => {
            let x = x.to_string();
            let resp = json!({
                "status": "error",
                "message": x
            });
            Json(resp)
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct AddBookingForm {
    room_pk: i32,
    date_start: String,
    date_end: String,
}

impl AddBookingForm {
    pub fn check(&self) -> bool {
        let mut is_valid = false;

        /*if let Ok(s) = DateTime::from_str(&self.date_start) {
            if let Ok(e) = DateTime::from_str(&self.date_end) {
                is_valid = true;
            }
        }*/

        is_valid
    }
}
