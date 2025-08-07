use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sea_orm::{DbErr, DeleteResult, ModelTrait};
use serde::Serialize;
use serde_json::json;
use entity::room::Model;
pub fn match_delete(result: Result<DeleteResult, DbErr>) -> impl IntoResponse {
    match result {
        Ok(x) => {
            let resp = Json(json!({
                "status": "success",
                "data": x.rows_affected > 0
            }));
            (StatusCode::OK, resp)
        }
        Err(err) => {
            let resp = Json(json!({
                "status": "error",
                "message": err.to_string(),
            }));
            (StatusCode::INTERNAL_SERVER_ERROR, resp)
        }
    }
}

pub fn match_get_one<T: ModelTrait + Serialize>(result: Result<Option<T>, DbErr>) -> impl IntoResponse {
    match result {
        Ok(r) => {
            if let Some(val) = r {
                let json = json!({
                   "status": "success",
                    "data": val
                });
                return (StatusCode::OK, Json(json));
            } else {
                let json = json!({
                   "status": "error",
                   "message": "Not found"
                });
                return (StatusCode::NOT_FOUND, Json(json));
            }
        }
        Err(err) => {
            let json = json!({
               "status": "error",
               "message": err.to_string()
            });
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json));
        }
    }
}

pub fn match_update(result: Result<impl ModelTrait, DbErr>) -> impl IntoResponse {
    match result {
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
    }
}