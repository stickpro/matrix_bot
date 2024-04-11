use serde::{Deserialize, Serialize};
use garde::Validate;
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct SendMessageToMatrix {
    #[garde(length(min = 3))]
    pub room_id: String,
    #[garde(length(min = 5))]
    pub message: String,
}

impl SendMessageToMatrix {
    pub fn new(room_id: &str, message: &str) -> Self {
        Self {
            room_id: room_id.to_string(),
            message: message.to_string(),
        }
    }
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}