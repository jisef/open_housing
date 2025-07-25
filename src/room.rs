use std::sync::Arc;
use axum::extract::{Query, State};
use axum::Json;
use axum::response::IntoResponse;
use sea_orm::ActiveModelTrait;
use serde_json::json;
use crate::App;
use crate::booking::BookingParams;
use crate::data_objects::database::room;
use crate::data_objects::request::room::AddRoom;

pub async fn get_rooms(
    State(app): State<Arc<App>>,
    Query(params): Query<BookingParams>,
) -> impl IntoResponse {

}

pub async fn add_rooms(
    State(app): State<Arc<App>>,
    Json(data): Json<AddRoom>,
) -> impl IntoResponse {

    let apple = room::ActiveModel {
        room_pk: Default::default(),
        number: Default::default(),
        name: Default::default(),
        capacity: Default::default(),
        max_capacity: Default::default(),
        is_apartment: Default::default(),
        has_kitchen: Default::default(),
        bedrooms: Default::default(),
    };
    let apple = apple.insert(&app.pool).await;






    Json(json!({
        "sigmal": true
    }))
}