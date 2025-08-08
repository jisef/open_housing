use crate::data_objects::request::room::AddRoom;
use crate::templates::{get_error_json, get_success_json, match_delete, match_get_one, match_update};
use crate::App;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::NaiveDate;
use entity::room::{ActiveModel, Entity as Room};
use entity::{booking, room};
use sea_orm::{ActiveModelBehavior, DbErr, DeleteResult, PaginatorTrait};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, Set};
use sea_orm::{Condition, DatabaseConnection};
use sea_query::ExprTrait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomParams {
    pub valid: Option<bool>,
    pub limit: Option<u64>,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

/// Gets rooms
///  If only date_end (to) is specified, it will be ignored
pub async fn get_rooms(
    State(app): State<Arc<App>>,
    Query(params): Query<RoomParams>,
) -> impl IntoResponse {
    let mut query: Result<Vec<room::Model>, DbErr> = Ok(vec![]);
    query = Room::find()
        .limit(params.limit.unwrap_or(crate::DEFAULT_LIMIT))
        .all(&app.connection)
        .await;

    match query {
        Ok(data) => {
            eprintln!("getting room successful");
            (StatusCode::OK, get_success_json(data))
        }
        Err(r) => {
            eprintln!("getting room failed");
            
            (StatusCode::INTERNAL_SERVER_ERROR, get_error_json(r.to_string()))
        }
    }
}

pub async fn add_room(State(app): State<Arc<App>>, Json(data): Json<AddRoom>) -> impl IntoResponse {
    let cloned = data.clone();
    let result = cloned.validate(&app.connection).await;

    match result {
        Ok(valid) => {
            if !valid {
                return (
                    StatusCode::BAD_REQUEST,
                    get_error_json("Invalid data")
                );
            }
        }
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                get_error_json(err)
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
            eprintln!("adding room failed");

            message = get_error_json(e.to_string());
            code = StatusCode::INTERNAL_SERVER_ERROR;
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
        if self.from >= self.to {
            return Err("from date must be before to date");
        }

        Ok(())
    }
}

/// return free as bool
pub async fn get_room_is_free(
    State(app): State<Arc<App>>,
    Query(params): Query<RoomIsFreeParams>,
) -> (StatusCode, Json<Value>) {
    let mut code = StatusCode::OK;
    let mut json_response = Json::default();
    let check_result = params.check();
    match check_result {
        Err(err) => {
            let json = get_error_json(err.to_string());
            eprintln!("checking room failed");
            return (StatusCode::INTERNAL_SERVER_ERROR, json);
        }
        _ => {}
    }

    let result = check_booking(&app.connection, params.from, params.to, vec![params.room]).await;

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
            let json = get_error_json(r.to_string());
            eprintln!("checking room failed");
            code = StatusCode::INTERNAL_SERVER_ERROR;
            json_response = json;
        }
    }
    (code, json_response)
}
/// check if a room is free in the given timespan
/// 'from' must be before 'to'
/// 'rooms': as a list of the primary keys
pub async fn check_booking(
    conn: &DatabaseConnection,
    from: NaiveDate,
    to: NaiveDate,
    rooms: Vec<i32>,
) -> Result<bool, String> {
    let result = entity::booking::Entity::find()
        .inner_join(entity::room_booking::Entity)
        .filter(entity::room_booking::Column::RoomFk.is_in(rooms))
        .filter(
            Condition::all()
                .add(booking::Column::DateEnd.gte(from))
                .add(booking::Column::DateStart.lte(to)),
        )
        .count(conn)
        .await;

    match result {
        Ok(data) => Ok(data == 0),
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
            ..Default::default()
        }
    }
}

pub async fn delete_room(
    State(app): State<Arc<App>>,
    Path(room_pk): Path<i32>,
) -> impl IntoResponse {
    // if a booking has a room it will fail
    let result: Result<DeleteResult, DbErr> = room::Entity::delete_by_id(room_pk)
        .exec(&app.connection)
        .await;

    let resp = match_delete(result);
    resp
}
