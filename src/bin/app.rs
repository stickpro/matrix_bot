
use futures::{FutureExt};
use tracing::info;
use matrix_bot::{configure, util};
use matrix_bot::constant::CONFIG;
use matrix_bot::error::AppResult;
use matrix_bot::server::AppServer;
use matrix_bot::server::worker::MessengerTask;

#[tokio::main]
async fn main() -> AppResult<()> {
    let _file_appender_guard = configure::tracing::init()?;
    info!("The initialization of Tracing was successful.");
    let config = CONFIG.clone();
    info!("Reading the config file was successful.");
    let server = AppServer::new(config.clone()).await?;
    info!("Create a new server.");
    let messenger = MessengerTask::new(server.state.clone());
    info!("Run the server.");

    util::task::join_all(vec![
        (true, server.run().boxed()),
        (true, messenger.run().boxed()),
    ]).await?;
    Ok(())
}





