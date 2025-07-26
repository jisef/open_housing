use crate::data_objects::database::booking::{
    ActiveModel, Column, Entity as Booking, Model as BookingModel,
};
use crate::{data_objects, App};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeDelta, Utc};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoIdentity, QueryFilter, QueryOrder, QuerySelect, QueryTrait, Set};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::sync::Arc;

pub async fn get_bookings(
    State(app): State<Arc<App>>,
    Query(params): Query<BookingParams>,
) -> impl IntoResponse {
    let utc: DateTime<Utc> = Utc::now();
    let mut now: DateTimeWithTimeZone = utc.into();
    let yesterday = now.sub(TimeDelta::new(86400 as i64, 0).unwrap());
    let tomorrow = now.add(TimeDelta::new(86400 as i64, 0).unwrap());


    let query = Booking::find()
        .apply_if(Some(params.limit), |mut query, v| {
            query.limit(v.unwrap() as u64)
        })
        .apply_if(Some(params.future), | mut query, v| {
            if let Some(val) = v {
                if val {
                    query.filter(Column::DateStart.gt(yesterday))
                } else {
                    query.filter(Column::DateEnd.lt(tomorrow))
                }
            } else {
                query.filter(Column::DateStart.gt(yesterday))
            }
        })
        .order_by_desc(Column::DateStart)
        .all(&app.connection).await;

    match query {
        Ok(bookings) => {
            let response = json!({
                "status": "success",
                "amount": bookings.len(),
                "data": bookings,
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
    pub limit: Option<i32>,
    pub date: Option<NaiveDate>,
    pub future: Option<bool>,
}

pub async fn add_booking(
    State(app): State<Arc<App>>,
    Json(params): Json<AddBookingData>,
) -> impl IntoResponse {
    params.check();

    //let utc: DateTime<Utc> = params.date_start.parse().unwrap();
    let start: DateTimeWithTimeZone = params.date_end.parse().unwrap();
    let end: DateTimeWithTimeZone = params.date_start.parse().unwrap();

    let booking = ActiveModel {
        date_start: Set(start),
        date_end: Set(end),
        with_breakfast: Set(Some(params.breakfast)),
        valid: Set(Some(true)),
        room_fk: Set(Some(params.room)),
        num_full_aged_guests: Set(Some(params.adults)),
        num_children: Set(Some(params.children)),
        checked_in: Set(Some(params.checked_in)),
        ..Default::default()
    };
    let result = booking.insert(&app.connection).await;

    let mut json: Json<Value> = Json::default();

    match result {
        Ok(x) => {
            let resp = json!({
                "status": "success",
                "booking_pk": x.booking_pk,
            });
            json = Json(resp);
        }
        Err(x) => {
            let x = x.to_string();
            let resp = json!({
                "status": "error",
                "message": x
            });
            json = Json(resp);
        }
    }
    (StatusCode::OK, json)
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct AddBookingData {
    room: i32,
    date_start: String,
    date_end: String,
    adults: i32,
    children: i32,
    checked_in: bool,
    breakfast: bool,
}

impl AddBookingData {
    pub fn check(&self) -> bool {
        let mut is_valid = true;

        /*if let Ok(s) = DateTime::from_str(&self.date_start) {
            if let Ok(e) = DateTime::from_str(&self.date_end) {
                is_valid = true;
            }
        }*/

        is_valid
    }
}
