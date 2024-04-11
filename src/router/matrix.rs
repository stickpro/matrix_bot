use axum::routing::{post};
use crate::handler::matrix;
use crate::server::state::AppState;

pub fn add_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
        .route("/api/v1/matrix/send_message", post(matrix::send_message))
}
