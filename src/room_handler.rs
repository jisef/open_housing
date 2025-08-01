use crate::data_objects::db::booking::Column as BookingColumn;
use crate::data_objects::db::{booking, room};
use crate::data_objects::db::room::{ActiveModel, Column, Entity as Room};
use crate::data_objects::request::room::AddRoom;
use crate::templates::{match_delete, match_get_one, match_update};
use crate::App;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::NaiveDate;
use sea_orm::{ActiveModelBehavior, DbErr, DeleteResult};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, QueryTrait, Set,
};
use sea_orm::{Condition, DatabaseConnection};
use sea_orm::{JoinType, RelationTrait};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;
use sea_query::Expr;

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomParams {
    pub valid: Option<bool>,
    pub limit: Option<i32>,
}

pub async fn get_rooms(
    State(app): State<Arc<App>>,
    Query(params): Query<RoomParams>,
) -> impl IntoResponse {
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
                query.filter(Column::RoomValid.eq(val))
            } else {
                query.filter(Column::RoomValid.eq(true))
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
        room_name: Set(Some(data.name)),
        capacity: Set(Some(data.capacity)),
        ..Default::default()
    };

    if let Some(max) = data.max_capacity {
        room.max_capacity = Set(Some(max));
    }
    if let Some(isApartment) = data.is_apartment {
        room.is_apartment = Set(Some(isApartment));
    }
    if let Some(hasKitchen) = data.has_kitchen {
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
    pub room: i32,
    pub from: NaiveDate,
    pub to: NaiveDate,
}

impl RoomIsFreeParams {
    pub fn check(&self) -> Result<(), &str> {
        Ok(()) //todo: implement
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
    from: NaiveDate,
    to: NaiveDate,
    room: i32,
) -> Result<bool, String> {
    let params = RoomIsFreeParams { room, from, to };

    let result = Room::find()
        .join(JoinType::InnerJoin, room::Relation::Booking.def())
        .filter(booking::Column::BookingValid.eq(true))
        .filter(Condition::all().add(booking::Column::DateEnd.gt(from)).add(booking::Column::DateStart.lt(to)))
        .filter(Column::RoomPk.eq(params.room))
        .all(conn)
        .await;
    match result {
        Ok(state) => Ok(state.len() == 0),
        Err(err) => Err(err.to_string()),
    }
}

/// gets a room through the PK
pub async fn get_room(State(app): State<Arc<App>>, Path(room_pk): Path<i32>) -> impl IntoResponse {
    let room = room::Entity::find_by_id(room_pk).one(&app.connection).await;

    let x = match_get_one::<room::Model>(room);
    x

    /*match room {
        Ok(r) => {
            if let Some(val) = r {
                let json = json!({
                   "status": "success",
                    "data": val
                });
                eprintln!("getting 1 room successful");
                (StatusCode::OK, Json(json))
            } else {
                let json = json!({
                   "status": "error",
                   "message": format!("Room {} not found", room_pk)
                });
                (StatusCode::NOT_FOUND, Json(json))
            }
        }
        Err(err) => {
            let json = json!({
               "status": "error",
               "message": err.to_string()
            });

            eprintln!("getting 1 room failed");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json))
        }
    }*/
}

pub async fn update_room(
    State(app): State<Arc<App>>,
    Path(room_pk): Path<i32>,
    Json(data): Json<PatchRoom>,
) -> impl IntoResponse {
    println!("updating room {} data {:?}", room_pk, data);
    let mut x = ActiveModel::new();
    x.room_pk = Set(room_pk);
    if let Some(val) = data.number {
        x.number = Set(Some(val));
    }
    if let Some(val) = data.name {
        x.room_name = Set(Some(val));
    }
    if let Some(val) = data.capacity {
        x.capacity = Set(Some(val));
    }
    if let Some(val) = data.maxCapacity {
        x.max_capacity = Set(Some(val));
    }
    if let Some(val) = data.isApartment {
        x.is_apartment = Set(Some(val));
    }
    if let Some(val) = data.hasKitchen {
        x.has_kitchen = Set(Some(val));
    }
    let result = x.update(&app.connection).await;
    let resp = match_update(result);
    resp

    /*match result {
        Ok(_) => {
            let resp = Json(json!({
                "status": "success",
                "data":true,
            }));
            (StatusCode::OK, resp)
        }
        Err(err) => {
            let resp = Json(json!({
                "status": "error",
                "message": err.to_string(),
            }));
            (StatusCode::OK, resp)
        }
    }*/
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PatchRoom {
    pub number: Option<i32>,
    pub name: Option<String>,
    pub capacity: Option<i32>,
    pub maxCapacity: Option<i32>,
    pub isApartment: Option<bool>,
    pub hasKitchen: Option<bool>,
    pub bedrooms: Option<i32>,
    pub valid: Option<bool>,
}

impl From<PatchRoom> for ActiveModel {
    fn from(value: PatchRoom) -> Self {
        ActiveModel {
            room_pk: Default::default(),
            number: Set(value.number),
            room_name: Set(value.name),
            capacity: Set(value.capacity),
            max_capacity: Set(value.maxCapacity),
            is_apartment: Set(value.isApartment),
            has_kitchen: Set(value.hasKitchen),
            bedrooms: Set(value.bedrooms),
            room_valid: Set(value.valid),
            ..Default::default()
        }
    }
}

pub async fn delete_room(
    State(app): State<Arc<App>>,
    Path(room_pk): Path<i32>,
) -> impl IntoResponse {
    let result: Result<DeleteResult, DbErr> = room::Entity::delete_by_id(room_pk)
        .exec(&app.connection)
        .await;

    let resp = match_delete(result);
    resp
}
