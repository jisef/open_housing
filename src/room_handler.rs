use crate::data_objects::db::booking::Column as BookingColumn;
use crate::data_objects::db::room;
use crate::data_objects::db::room::{ActiveModel, Column, Entity as Room};
use crate::data_objects::request::room::AddRoom;
use crate::App;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::NaiveDate;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, QueryTrait, Set,
};
use sea_orm::{Condition, DatabaseConnection};
use sea_orm::{JoinType, RelationTrait};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomParams {
    pub valid: Option<bool>,
    pub limit: Option<i32>,
}

pub async fn get_rooms(
    State(app): State<Arc<App>>,
    Query(params): Query<RoomParams>,
) -> impl IntoResponse {
    //TODO: implement valid
    let query = Room::find()
        .apply_if(Some(params.limit), |query, v| {
            if let Some(val) = v {
                query.limit(val as u64)
            } else {
                query.limit(100u64)
            }
        })
        .apply_if(Some(params.valid), |query, v| {
            if let Some(val) = v {
                query.filter(Column::Valid.eq(val))
            } else {
                query.filter(Column::Valid.eq(true))
            }
        })
        .all(&app.connection)
        .await;
    match query {
        Ok(data) => {
            let json = json!({
                "status": "success",
                "data": data
            });
            eprintln!("getting room successful");
            Json(json)
        }
        Err(r) => {
            let json = json!({
                "status": "error",
                "message": r.to_string()
            });
            eprintln!("getting room failed");
            Json(json)
        }
    }
}

pub async fn add_rooms(
    State(app): State<Arc<App>>,
    Json(data): Json<AddRoom>,
) -> impl IntoResponse {
    let cloned = data.clone();
    let result = cloned.validate(&app.connection).await;

    match result {
        Ok(valid) => {
            if !valid {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"status": "error", "message": "Invalid data"})),
                );
            }
        }
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": err})),
            );
        }
    }

    let mut room = ActiveModel {
        number: Set(Some(data.number)),
        name: Set(Some(data.name)),
        capacity: Set(Some(data.capacity)),
        ..Default::default()
    };

    if let Some(max) = data.maxCapacity {
        room.max_capacity = Set(Some(max));
    }
    if let Some(isApartment) = data.isApartment {
        room.is_apartment = Set(Some(isApartment));
    }
    if let Some(hasKitchen) = data.hasKitchen {
        room.has_kitchen = Set(Some(hasKitchen));
    }
    if let Some(bedrooms) = data.bedrooms {
        room.bedrooms = Set(Some(bedrooms));
    }
    let data = room.insert(&app.connection).await;
    let mut code: StatusCode = StatusCode::OK.into();
    let mut message: Json<_> = Json::default();
    match data {
        Ok(x) => {
            let json = json!({
                "status": "success",
                "room_pk": x
            });
            code = StatusCode::CREATED;
            eprintln!("Added Room");
            message = json.into();
        }
        Err(e) => {
            let mes = json!({
                "status": "error",
                "message": e.to_string()
            });
            eprintln!("adding room failed");
            code = StatusCode::INTERNAL_SERVER_ERROR;
            message = mes.into();
        }
    }
    (code, message)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomIsFreeParams {
    /// room_pk
    pub room: Option<i32>,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

impl RoomIsFreeParams {
    pub fn check(&self) -> Result<(), &str> {
        if self.from.is_some() && self.to.is_some() || self.room.is_some() {
            Ok(())
        } else {
            Err("Parameters not valid")
        }
    }
}

/// return free as bool
pub async fn get_room_is_free(
    State(app): State<Arc<App>>,
    Query(params): Query<RoomIsFreeParams>,
) -> (StatusCode, Json<Value>) {
    let mut code = StatusCode::OK;
    let mut json_response = Json::default();

    let result = check_booking(&app.connection, params.from, params.to, params.room).await;

    match result {
        Ok(data) => {
            let json = json!({
                "status": "success",
                "free": data
            });
            eprintln!("checking room: {}", data);
            json_response = Json(json);
        }
        Err(r) => {
            let json = json!({
                "status": "error",
                "message": r.to_string()
            });
            eprintln!("checking room failed");
            code = StatusCode::INTERNAL_SERVER_ERROR;
            json_response = Json(json);
        }
    }
    (code, json_response)
}

/// from can be after to - check if a room is free in the given timespan
pub async fn check_booking(
    conn: &DatabaseConnection,
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
    room: Option<i32>,
) -> Result<bool, String> {
    let params = RoomIsFreeParams { room, from, to };

    let result = Room::find()
        .join(JoinType::InnerJoin, room::Relation::Booking.def())
        .apply_if(Some(params.room), |query, v| {
            if let Some(val) = v {
                query.filter(Column::RoomPk.eq(val))
            } else {
                query
            }
        })
        .apply_if(Some(params.from), |query, v| {
            if let Some(from) = v {
                if let Some(to) = params.to {
                    query.filter(
                        Condition::any()
                            .add(
                                Condition::any()
                                    .add(BookingColumn::DateStart.between(from, to))
                                    .add(BookingColumn::DateStart.is_in(vec![from, to])),
                            )
                            .add(
                                Condition::any()
                                    .add(BookingColumn::DateEnd.between(from, to))
                                    .add(BookingColumn::DateEnd.is_in(vec![from, to])),
                            ),
                    )
                } else {
                    query.filter(BookingColumn::DateStart.eq(from))
                }
            } else {
                query
            }
        })
        .all(conn)
        .await;
    match result {
        Ok(state) => Ok(state.len() == 0),
        Err(err) => Err(err.to_string()),
    }
}
