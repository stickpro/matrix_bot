use std::time::Duration;
use matrix_sdk::Client;
use matrix_sdk::config::SyncSettings;
use matrix_sdk::room::Room;
use matrix_sdk::ruma::events::room::member::StrippedRoomMemberEvent;
use tokio::time::sleep;
use tracing::{error, info};
use crate::configure::matrix::MatrixConfig;
use crate::error::AppResult;

pub type MatrixClient = Client;

pub trait MatrixClientExt: Sized {
    fn build_from_config(config: &MatrixConfig) -> impl std::future::Future<Output=AppResult<Self>>;
}

impl MatrixClientExt for MatrixClient {
    async fn build_from_config(config: &MatrixConfig) -> AppResult<Self> {
        let client = {
            let builder = Client::builder()
                .homeserver_url(config.home_server.clone());

            builder.build().await?
        };

        client.login_username(&config.login, &config.password)
            .initial_device_display_name("Resender")
            .send()
            .await?;

        client.add_event_handler(auto_join_room);
        client.sync_once(SyncSettings::default()).await.unwrap();

        Ok(client)
    }
}

async fn auto_join_room(
    room_member: StrippedRoomMemberEvent,
    client: Client,
    room: Room,
) {
    if room_member.state_key != client.user_id().unwrap() {
        return;
    }

    if let Room::Invited(room) = room {
        tokio::spawn(async move {
            info!("Autojoining room {}", room.room_id());
            let mut delay = 2;

            while let Err(err) = room.accept_invitation().await {
                error!("Failed to join room {} ({err:?}), retrying in {delay}s", room.room_id());

                sleep(Duration::from_secs(delay)).await;
                delay *= 2;

                if delay > 3600 {
                    error!("Can't join room {} ({err:?})", room.room_id());
                    break;
                }
            }
            info!("Successfully joined room {}", room.room_id());
        });
    }
}