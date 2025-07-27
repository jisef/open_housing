use crate::booking::BookingParams;
use crate::data_objects::db::room;
use crate::data_objects::db::room::{
    ActiveModel, Column, Entity as Room, Model as RoomModel, Model,
};
use crate::data_objects::request::room::AddRoom;
use crate::App;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, QueryTrait, Set};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
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
        .apply_if(Some(params.limit), |mut query, v| {
            if let Some(val) = v {
                query.limit(val as u64)
            } else {
                query.limit(100u64)
            }
        }).apply_if(Some(params.valid), |mut query, v| {
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
            eprintln!("getting room succesful");
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
