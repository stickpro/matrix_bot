
use crate::{server::state::AppState};
use axum::Router;

pub mod server;
pub mod matrix;

pub fn create_router_app(state: AppState) -> Router {
    let router = Router::new();
    let router = server::add_routers(router);
    let router = matrix::add_routers(router);
    router.with_state(state)

}