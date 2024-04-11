use axum::Json;
use crate::dto::response::{MessageResponse, ServiceStatusResponse};
use crate::error::AppResult;

// Health check
pub async fn health_check() -> AppResult<Json<MessageResponse>> {
    Ok(Json(MessageResponse::new("Ok")))
}

pub async fn server_state() -> AppResult<Json<ServiceStatusResponse>> {
    let resp = ServiceStatusResponse {
        matrix: true,
    };
    Ok(Json(resp))
}