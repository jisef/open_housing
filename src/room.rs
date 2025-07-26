use crate::booking::BookingParams;
use crate::data_objects::database::room::ActiveModel;
use crate::data_objects::request::room::AddRoom;
use crate::App;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sea_orm::{ActiveModelTrait, Set};
use serde_json::{json, Value};
use std::sync::Arc;

pub async fn get_rooms(
    State(app): State<Arc<App>>,
    Query(params): Query<BookingParams>,
) -> impl IntoResponse {
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
            message = json.into();
        },
        Err(e) => {
            let mes = json!({
                "status": "error",
                "message": e.to_string()
            });
            code = StatusCode::INTERNAL_SERVER_ERROR;
            message = mes.into();
        }
    }
    (code, message)

}
