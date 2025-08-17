use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sea_orm::{DbErr, EntityTrait, InsertResult, Set};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use entity::user::ActiveModel;
use crate::App;
use crate::auth_handler::{Backend, User};
use crate::templates::{get_error_json, get_success_json};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
}

pub async fn add_user(
    State(app): State<Arc<App>>,
    Json(params): Json<UserRequest>,
) -> impl IntoResponse {
    let data = ActiveModel {
        username: Set(params.username),
        password: Set(params.password),
        ..Default::default()
    };
    let result = entity::user::Entity::insert(data).exec(&app.connection).await;

    match result {
        Ok(_) => {
            (StatusCode::CREATED, get_success_json("User added successfully"))
        }
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, get_error_json(err.to_string()))
        }
    }
}

