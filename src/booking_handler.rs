use crate::data_objects::db::booking::{
    ActiveModel, Column, Entity as Booking, Model as BookingModel, Model,
};
use crate::data_objects::db::prelude::Room;
use crate::data_objects::db::room;
use crate::room_handler::RoomIsFreeParams;
use crate::{room_handler, App};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{Date, DateTime, NaiveDate, TimeDelta, Utc};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbBackend, DbErr, EntityOrSelect,
    EntityTrait, FromQueryResult, QueryFilter, QueryOrder, QuerySelect, QueryTrait, Set, Statement,
    StatementBuilder,
};
use sea_query::JoinType;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::sync::Arc;

const DEFAULT_LIMIT: u64 = 100u64;

pub async fn get_bookings(
    State(app): State<Arc<App>>,
    Query(params): Query<BookingParams>,
) -> impl IntoResponse {
    let utc: DateTime<Utc> = Utc::now();
    let mut now: DateTimeWithTimeZone = utc.into();
    let yesterday = std::ops::Sub::sub(now, TimeDelta::new(86400 as i64, 0).unwrap());
    let tomorrow = std::ops::Add::add(now, TimeDelta::new(86400 as i64, 0).unwrap());

    let query = Booking::find()
        .apply_if(Some(params.limit), |mut query, v| {
            if let Some(limit) = v {
                query.limit(limit as u64)
            } else {
                query.limit(10)
            }
        })
        .apply_if(Some(params.future), |mut query, v| {
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
        .all(&app.connection)
        .await;

    match query {
        Ok(bookings) => {
            let response = json!({
                "status": "success",
                "amount": bookings.len(),
                "data": bookings,
            });
            eprintln!("Getting booking successful");
            Json(response)
        }
        Err(e) => {
            eprintln!("Database error: When getting room: {}", e);

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
    let mut json: Json<Value> = Json::default();

    let x = params.check(&app.connection).await;
    if !x {
        json = Json(json!({
            "status": "error",
            "message": "Booking not valid",
        }));
        return (StatusCode::BAD_REQUEST, json);
    }

    let start: NaiveDate = params.date_end;
    let end: NaiveDate = params.date_start;
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

    match result {
        Ok(x) => {
            let resp = json!({
                "status": "success",
                "booking_pk": x.booking_pk,
            });
            eprintln!("Add booking successful");
            json = Json(resp);
        }
        Err(x) => {
            let x = x.to_string();
            let resp = json!({
                "status": "error",
                "message": x
            });
            eprintln!("Add booking failed");
            json = Json(resp);
        }
    }
    (StatusCode::OK, json)
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct AddBookingData {
    room: i32,
    date_start: NaiveDate,
    date_end: NaiveDate,
    adults: i32,
    children: i32,
    checked_in: bool,
    breakfast: bool,
}

impl AddBookingData {
    pub async fn check(&self, conn: &DatabaseConnection) -> bool {
        let mut is_valid = true;

        // check date
        if self.date_start >= self.date_end {
            is_valid = false;
        }

        // check room is free
        let result = room_handler::check_booking(
            conn,
            Some(self.date_start),
            Some(self.date_end),
            Some(self.room),
        )
        .await;
        if let Ok(r) = result {
            is_valid = r && is_valid;
        } else {
            is_valid = false;
        }

        // check num guests
        if self.adults + self.children == 0 {
            is_valid = false;
        }

        is_valid
    }
}

#[derive(Deserialize)]
pub struct GetBookingTodayParams {
    pub arrival: Option<bool>,
    pub limit: Option<i32>,
}

pub async fn get_bookings_today(
    State(app): State<Arc<App>>,
    Query(params): Query<GetBookingTodayParams>,
) -> impl IntoResponse {
    // TODO add arrival
    let current_date = Utc::now().naive_utc();
    let current_date = NaiveDate::from(current_date);
    let mut column = Column::DateStart;
    if let Some(x) = params.arrival {
        if !x {
            column = Column::DateEnd;
        }
    }

    let result = Booking::find()
        .apply_if(Some(params.limit), |query, v| {
            if let Some(limit) = v {
                query.limit(limit as u64)
            } else {
                query.limit(DEFAULT_LIMIT)
            }
        })
        .filter(column.eq(current_date))
        .all(&app.connection)
        .await;

    let mut code = StatusCode::OK;
    let mut return_json: Json<Value> = Json::default();
    match result {
        Ok(data) => {
            let json = json!({
                "status": "success",
                "data": data,
            });
            return_json = Json(json);
        }
        Err(error) => {
            let json = json!({
                "status": "error",
                "message": error.to_string()
            });
            code = StatusCode::INTERNAL_SERVER_ERROR;
            return_json = Json(json);
        }
    }

    (code, return_json)
}
