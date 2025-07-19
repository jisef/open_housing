use crate::data_objects::database::overview::OverviewData;
use crate::App;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

pub async fn get_overview(
    State(app): State<Arc<App>>,
    Query(params): Query<OverviewParams>,
) -> impl IntoResponse {
    if params.check() {
        let sql = format!("SELECT * FROM view_bookingoverview WHERE date_start BETWEEN '{}' and '{}' OR date_end BETWEEN '{}' and '{}'", params.clone().start_date.unwrap(), params.clone().end_date.unwrap(), params.start_date.unwrap(), params.end_date.unwrap());
        let result = sqlx::query_as::<_, OverviewData>(&*sql)
            .fetch_all(&app.pool)
            .await;
        match result {
            Ok(j) => {
                let response = json!({
                    "success": true,
                    "data": j
                });
                Json(response)
            }
            Err(e) => {
                let response = json!({
                    "success": false,
                    "message": e.to_string()
                });
                Json(response)
            }
        }
    } else {
        let response = json!({
            "status": "error",
            "message": "Invalid Arguments"
        });

        Json(response)
    }
}

#[derive(Deserialize,  Clone)]
pub struct OverviewParams {
    start_date: Option<String>,
    end_date: Option<String>,
}
impl OverviewParams {
    /// Checks if numbers are available
    pub fn check(&self) -> bool {
        if self.start_date.is_some() && self.end_date.is_some() {
            true
        } else {
            false
        }
    }
}
