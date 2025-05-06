use axum::{Router, Json, routing::post};

use crate::redis::store_checkin;
use crate::models::ClockInRequest;

pub fn routes() -> Router {
    Router::new()
        .route("/api/clockin", post(clockin))
        .route("/api/clockout", post(clockout))
}

async fn clockin(Json(payload): Json<ClockInRequest>) -> Json<serde_json::Value> {
    store_checkin(&payload.access_key, payload.project_id).await;

    Json(serde_json::json!({
        "status": "Clocked in",
        "project_id": payload.project_id
    }))
}

async fn clockout(Json(payload): Json<ClockInRequest>) -> Json<serde_json::Value> {
    // TODO: Fetch from Redis, write to DB
    Json(serde_json::json!({
        "status": "Clocked out",
        "summary": payload.project_id
    }))
}