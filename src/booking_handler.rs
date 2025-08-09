use crate::templates::{get_error_json, get_success_json, match_delete, match_update};
use crate::{room_handler, App, DEFAULT_LIMIT};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{NaiveDate, Utc};
use entity::booking::{ActiveModel, Column, Entity as Booking, Entity};
use entity::room_booking;
use entity::{booking, room};
use itertools::Itertools;
use sea_orm::prelude::Date;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter, QueryOrder, QuerySelect, QueryTrait, RelationTrait, Select, Set};
use sea_query::JoinType;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use std::thread::Thread;
/// Joins booking and room<br>
/// remember to parse to a custom model
pub fn get_booking_query() -> Select<Entity> {
    Booking::find()
        .join(JoinType::InnerJoin, booking::Relation::RoomBooking.def())
        .join(JoinType::InnerJoin, room_booking::Relation::Room.def())
        .column_as(room::Column::RoomName, "room_name")
        .column_as(room::Column::Number, "number")
        .column_as(room::Column::Capacity, "capacity")
        .column_as(room::Column::MaxCapacity, "max_capacity")
        .column_as(room::Column::IsApartment, "is_apartment")
        .column_as(room::Column::HasKitchen, "has_kitchen")
        .column_as(room::Column::RoomPk, "room_pk")
        .column_as(room::Column::Bedrooms, "bedrooms")
}

/// Groups the result of
pub fn group_bookingview(bookings: Vec<BookingView>) -> Vec<BookingResponse> {
    let mut grouped_bookings: Vec<BookingResponse> = vec![];

    for (_, chunk) in &bookings.into_iter().chunk_by(|x| x.booking_pk) {
        let mut response: BookingResponse = BookingResponse {
            rooms: vec![],
            booking_pk: 0,
            date_start: Default::default(),
            date_end: Default::default(),
            with_breakfast: None,
            num_full_aged_guests: None,
            num_children: None,
            checked_in: None,
            created_at: None,
            checked_out: None,
        };
        let mut room_response: Vec<BookingRoomResponse> = vec![];
        let mut is_first = true;
        for booking in chunk {
            let room = BookingRoomResponse {
                bedrooms: booking.bedrooms,
                room_name: booking.room_name,
                max_capacity: booking.max_capacity,
                is_apartment: booking.is_apartment,
                has_kitchen: booking.has_kitchen,
                capacity: booking.capacity,
                room_pk: booking.room_pk,
                number: booking.number,
            };
            room_response.push(room);
            if is_first {
                response.booking_pk = booking.booking_pk;
                response.date_start = booking.date_start;
                response.date_end = booking.date_end;
                response.with_breakfast = booking.with_breakfast;
                response.num_full_aged_guests = booking.num_full_aged_guests;
                response.num_children = booking.num_children;
                response.checked_in = booking.checked_in;
                response.created_at = booking.created_at;
                response.checked_out = booking.checked_out;
            }
            is_first = false;
        }
        response.rooms = room_response;
        grouped_bookings.push(response);
    }
    grouped_bookings
}

pub async fn get_bookings(
    State(app): State<Arc<App>>,
    Query(params): Query<BookingParams>,
) -> impl IntoResponse {
    let result = get_booking_query()
        .limit(params.limit.unwrap_or(DEFAULT_LIMIT))
        .apply_if(Some(params.room_pk), |query, v| {
            if let Some(pk) = v{
                query.filter(room_booking::Column::RoomFk.eq(pk)).order_by_desc(booking::Column::CreatedAt)
            }
            else {
                query
            }
        })
        .into_model::<BookingView>()
        .all(&app.connection)
        .await;

    match result {
        Ok(data) => {
            let response = group_bookingview(data);
            (StatusCode::OK, get_success_json(response))
        }
        Err(error) => {
            (StatusCode::INTERNAL_SERVER_ERROR, get_error_json(error.to_string()))
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct BookingResponse {
    pub rooms: Vec<BookingRoomResponse>,
    pub booking_pk: i32,
    pub date_start: Date,
    pub date_end: Date,
    pub with_breakfast: Option<bool>,
    pub num_full_aged_guests: Option<i32>,
    pub num_children: Option<i32>,
    pub checked_in: Option<bool>,
    pub created_at: Option<sea_orm::prelude::DateTime>,
    pub checked_out: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BookingRoomResponse {
    pub bedrooms: Option<i32>,
    pub room_name: Option<String>,
    pub max_capacity: Option<i32>,
    pub is_apartment: Option<bool>,
    pub has_kitchen: Option<bool>,
    pub capacity: Option<i32>,
    pub room_pk: i32,
    pub number: Option<i32>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BookingParams {
    pub limit: Option<u64>,
    pub room_pk: Option<u64>,
}

pub async fn add_booking(
    State(app): State<Arc<App>>,
    Json(params): Json<AddBookingData>,
) -> impl IntoResponse {
    let mut json: Json<Value> = Json::default();

    let x = params.check(&app.connection).await;
    match x {
        Ok(x) => {
            if !x {
                return (StatusCode::BAD_REQUEST, get_error_json("Booking not valid"));
            }
        }
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, get_error_json(err));
        }
    }

    let start: NaiveDate = params.date_start;
    let end: NaiveDate = params.date_end;
    let booking = ActiveModel {
        date_start: Set(start),
        date_end: Set(end),
        with_breakfast: Set(Some(params.with_breakfast)),
        num_full_aged_guests: Set(params.num_full_aged_guests),
        num_children: Set(params.num_children),
        checked_in: Set(Some(params.checked_in)),
        ..Default::default()
    };
    // inserts the booking (without rooms)
    let result = booking.insert(&app.connection).await;

    match result {
        Ok(x) => {
            let resp = json!({
                "status": "success",
                "booking_pk": x.booking_pk,
            });
            let mut models = vec![];
            //TODO implement num people
            for y in params.rooms {
                let temp = room_booking::ActiveModel {
                    booking_fk: Set(x.booking_pk),
                    room_fk: Set(y.room_pk),
                    num_people: Default::default(),
                };
                models.push(temp);
            }

            // inserts rooms to the booking
            let result = entity::room_booking::Entity::insert_many(models)
                .exec(&app.connection)
                .await;
            match result {
                Ok(_) => {
                    eprintln!("Add booking successful");
                    json = Json(resp);
                }
                Err(err) => {
                    json = get_error_json(err.to_string());
                }
            }
        }
        Err(x) => {
            eprintln!("Add booking failed");
            json = get_error_json(x.to_string());
        }
    }
    (StatusCode::OK, json)
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct AddBookingData {
    date_start: NaiveDate,
    date_end: NaiveDate,
    num_full_aged_guests: Option<i32>,
    num_children: Option<i32>,
    checked_in: bool,
    with_breakfast: bool,
    booking_pk: Option<i32>,
    rooms: Vec<BookingRoomResponse>,
}

impl AddBookingData {
    pub async fn check(&self, conn: &DatabaseConnection) -> Result<bool, String > {
        let mut is_valid: bool = true;

        // check date
        if self.date_start >= self.date_end {
            is_valid = false;
        }

        // check if the rooms exist
        let list_room_pks: Vec<i32> = self.rooms.iter().map(|x| x.room_pk).collect();
        let list_room_pks_len: usize = list_room_pks.len();
        let result = entity::room::Entity::find()
            .filter(entity::room::Column::RoomPk.is_in(list_room_pks.clone()))
            .all(conn)
            .await;
        match result {
            Ok(x) => {
                is_valid = x.len() == list_room_pks_len;
            }
            Err(_) => {
                is_valid = false;
                return Err("Rooms not found".into());
            }
        }

        // check room is free
        let result =
            room_handler::check_booking(conn, self.date_start, self.date_end, list_room_pks).await;
        match result {
            Ok(r) => {
                is_valid = r && is_valid;
            }
            Err(err) => {
                return Err(err);
            }
        }

        // check num guests
        if self.num_full_aged_guests.or(Some(0)).unwrap() + self.num_children.or(Some(0)).unwrap()
            == 0
        {
            is_valid = false;
        }

        Ok(is_valid)
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

    match result {
        Ok(data) => {
            (StatusCode::OK, get_success_json(group_bookingview(data)))
        }
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, get_error_json(err.to_string()))
        }
    }
}

#[derive(Serialize, FromQueryResult, Debug)]
pub struct BookingView {
    pub number: Option<i32>,
    pub room_name: Option<String>,
    pub capacity: Option<i32>,
    pub max_capacity: Option<i32>,
    pub is_apartment: Option<bool>,
    pub has_kitchen: Option<bool>,
    pub bedrooms: Option<i32>,
    pub booking_pk: i32,
    pub date_start: Date,
    pub date_end: Date,
    pub with_breakfast: Option<bool>,
    pub num_full_aged_guests: Option<i32>,
    pub num_children: Option<i32>,
    pub checked_in: Option<bool>,
    pub created_at: Option<sea_orm::prelude::DateTime>,
    pub checked_out: Option<bool>,
    pub room_pk: i32,
}

#[derive(Serialize, Deserialize, FromQueryResult, Debug)]
pub struct RoomView {
    pub name: Option<String>,
    pub number: i32,
    pub room_pk: i32,
}

pub async fn get_booking(
    State(app): State<Arc<App>>,
    Path(booking_pk): Path<i32>,
) -> impl IntoResponse {
    let result = get_booking_query()
        .filter(booking::Column::BookingPk.eq(booking_pk))
        .into_model::<BookingView>()
        .all(&app.connection)
        .await;

    match result {
        Ok(data) => {
            let json = get_success_json(group_bookingview(data));
            (StatusCode::OK, json)
        }
        Err(err) => {
            let json = get_error_json(err.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, json)
        }
    }
}

pub async fn delete_booking(
    State(app): State<Arc<App>>,
    Path(booking_pk): Path<i32>,
) -> impl IntoResponse {
    let result = entity::room_booking::Entity::delete_many()
        .filter(entity::room_booking::Column::BookingFk.eq(booking_pk))
        .exec(&app.connection)
        .await;
    if result.is_err() {
        let resp = match_delete(result);
        return resp;
    }

    let result = Booking::delete_by_id(booking_pk)
        .exec(&app.connection)
        .await;
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
