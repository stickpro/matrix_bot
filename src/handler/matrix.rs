use axum::Json;
use crate::dto::response::ServiceStatusResponse;
use crate::error::{AppResult};
use crate::server::state::AppState;
use axum::extract::State;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use matrix_sdk::ruma::{RoomId};
use crate::dto::request::SendMessageToMatrix;


pub async fn send_message(State(state): State<AppState>, Json(req): Json<SendMessageToMatrix>) -> AppResult<Json<ServiceStatusResponse>> {
    let room_id = RoomId::parse(req.room_id).expect("");
    let room = state.matrix.get_joined_room(&room_id).unwrap();
    let content =  RoomMessageEventContent::text_plain(req.message.clone());

    room.send(content, None).await.expect("TODO: panic message");

    let resp = ServiceStatusResponse {
        matrix: true,
    };

    Ok(Json(resp))
}