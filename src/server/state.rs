use std::sync::Arc;
use tokio::sync::Notify;
use crate::configure::AppConfig;
use crate::error::AppResult;
use crate::matrix::{MatrixClient, MatrixClientExt};

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub messenger_notify: Arc<Notify>,
    pub matrix: Arc<MatrixClient>
}

impl AppState {
    pub async fn new(config: AppConfig) -> AppResult<Self> {
        let matrix = Arc::new(MatrixClient::build_from_config(&config.matrix).await?);
        Ok(Self {
            config: Arc::new(config),
            messenger_notify: Default::default(),
            matrix,
        })
    }
}