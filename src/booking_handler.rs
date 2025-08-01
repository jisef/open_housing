use crate::data_objects::db::booking::{ActiveModel, Column, Entity as Booking, Entity, Model};
use crate::data_objects::db::{booking, room};
use crate::{room_handler, App};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{DateTime, NaiveDate, TimeDelta, Utc};
use sea_orm::prelude::{Date, DateTimeWithTimeZone};
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, FromQueryResult, ModelTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait, RelationTrait, Select, Set};
use sea_query::JoinType;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::sync::Arc;
use crate::data_objects::db::prelude::Room;
use crate::room_handler::PatchRoom;
use crate::templates::{match_delete, match_get_one, match_update};

const DEFAULT_LIMIT: u64 = 100u64;

/// remember to parse to a custom model
pub fn get_booking_query() -> Select<Entity> {
    Booking::find()
        .join(JoinType::InnerJoin, booking::Relation::Room.def())
        .column_as(room::Column::RoomName, "room_name")
        .column_as(room::Column::Number, "room_number")
}

pub async fn get_bookings(
    State(app): State<Arc<App>>,
    Query(params): Query<BookingParams>,
) -> impl IntoResponse {
    let utc: DateTime<Utc> = Utc::now();
    let now: DateTimeWithTimeZone = utc.into();
    let yesterday = std::ops::Sub::sub(now, TimeDelta::new(86400 as i64, 0).unwrap());
    let tomorrow = std::ops::Add::add(now, TimeDelta::new(86400 as i64, 0).unwrap());

    let query = get_booking_query()
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
        .order_by_desc(Column::CreatedAt)
        .into_model::<BookingView>()
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
        booking_valid: Set(Some(true)),
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
        if self.date_start < self.date_end {
            is_valid = false;
        }

        // check if room exists and is valid
        let result = Room::find_by_id(self.room).filter(room::Column::RoomValid.eq(true)).one(conn).await;
        match result {
            Ok(x) => {
                is_valid = x.is_some();
            }
            Err(err) => {
                is_valid  = false;
            }
        }

        // check room is free
        let result = room_handler::check_booking(
            conn,
            self.date_start,
            self.date_end,
            self.room,
        ).await;

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

    let result = get_booking_query()
        .apply_if(Some(params.limit), |query, v| {
            if let Some(limit) = v {
                query.limit(limit as u64)
            } else {
                query.limit(DEFAULT_LIMIT)
            }
        })
        .filter(column.eq(current_date))
        .into_model::<BookingView>()
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

#[derive(Serialize, FromQueryResult, Debug)]
pub struct BookingView {
    #[sea_orm(primary_key)]
    pub booking_pk: i32,
    pub date_start: Date,
    pub date_end: Date,
    pub with_breakfast: Option<bool>,
    pub booking_valid: Option<bool>,
    pub room_fk: Option<i32>,
    pub num_full_aged_guests: Option<i32>,
    pub num_children: Option<i32>,
    pub checked_in: Option<bool>,
    pub created_at: Option<sea_orm::prelude::DateTime>,
    pub checked_out: Option<bool>,
    pub room_number: Option<i32>,
    pub room_name: Option<String>,
}

pub async fn get_booking(
    State(app): State<Arc<App>>,
    Path(booking_pk): Path<i32>,
) -> impl IntoResponse {
    let result = Booking::find_by_id(booking_pk).one(&app.connection).await;
    let resp = match_get_one::<Model>(result);
    resp
}

pub async fn delete_booking(
    State(app): State<Arc<App>>,
    Path(booking_pk): Path<i32>,
) -> impl IntoResponse {
    let result = Booking::delete_by_id(booking_pk).exec(&app.connection).await;
    let resp = match_delete(result);
    resp
}

pub async fn patch_booking(
    State(app): State<Arc<App>>,
    Path(booking_pk): Path<i32>,
    Json(data): Json<PatchBooking>,
) -> impl IntoResponse {
    let mut x = ActiveModel::new();
    x.booking_pk = Set(booking_pk);
    if let Some(val) = data.date_start {
        x.date_start = Set(val);
    }
    if let Some(val) = data.date_end {
        x.date_end = Set(val);
    }
    if let Some(val) = data.with_breakfast {
        x.with_breakfast = Set(Some(val));
    }
    if let Some(val) = data.booking_valid {
        x.booking_valid = Set(Some(val));
    }
    if let Some(val) = data.room_fk {
        x.room_fk = Set(Some(val))
    }
    if let Some(val) = data.num_full_aged_guests {
        x.num_full_aged_guests = Set(Some(val))
    }
    if let Some(val) = data.num_children {
        x.num_children = Set(Some(val))
    }
    if let Some(val) = data.checked_in {
        x.checked_in = Set(Some(val))
    }
    if let Some(val) = data.checked_out {
        x.checked_out = Set(Some(val))
    }
    let result = x.update(&app.connection).await;

    let resp = match_update(result);
    resp
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct PatchBooking {
    pub date_start: Option<Date>,
    pub date_end: Option<Date>,
    pub with_breakfast: Option<bool>,
    pub booking_valid: Option<bool>,
    pub room_fk: Option<i32>,
    pub num_full_aged_guests: Option<i32>,
    pub num_children: Option<i32>,
    pub checked_in: Option<bool>,
    pub checked_out: Option<bool>,
}

