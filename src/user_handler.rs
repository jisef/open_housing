use crate::auth_handler::{Backend, User};
use crate::templates::{get_error_json, get_success_json};
use crate::App;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use entity::user::ActiveModel;
use sea_orm::{DbErr, EntityTrait, InsertResult, Set};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
}

pub const HASH_COST: u32 = 12;
pub async fn add_user(
    State(app): State<Arc<App>>,
    Json(params): Json<UserRequest>,
) -> impl IntoResponse {
    let data = ActiveModel {
        username: Set(params.username),
        password: Set(
            bcrypt::hash(params.password, HASH_COST).expect("ERROR: Could not hash password")
        ),
        ..Default::default()
    };
    let result = entity::user::Entity::insert(data)
        .exec(&app.connection)
        .await;

    match result {
        Ok(_) => (
            StatusCode::CREATED,
            get_success_json("User added successfully"),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            get_error_json(err.to_string()),
        ),
    }
}
