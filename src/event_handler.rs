use std::str::FromStr;

use ethcontract::Event;
use strum::EnumString;
use tracing::info;

use crate::{entity_manager::entity_manager::event_data::ManageEntity, AppResult};

// TODO: this list is not exhaustive
// https://github.com/AudiusProject/audius-protocol/blob/27d92c574ccc1b445e408cca7e541711ccb0eca0/discovery-provider/src/tasks/entity_manager/utils.py
#[derive(Debug, EnumString)]
pub enum Events {
    // Playlist Events
    CreatePlaylist(),
    UpdatePlaylist(),
    DeletePlaylist(),
    // Track Events
    CreateTrack(),
    UpdateTrack(),
    DeleteTrack(),
    SaveTrack(),
    // User Events
    CreateUser(),
    UpdateUser(),
    VerifyUser(),
    FollowUser(),
    UpdateUserReplicaSet(),
    // Notification Events
    ViewNotification(),
    CreateNotification(),
    ViewPlaylistNotification(),
}

impl TryFrom<Event<ManageEntity>> for Events {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Event<ManageEntity>) -> AppResult<Self> {
        let event = &format!("{}{}", value.data.action, value.data.entity_type);
        let event = Events::from_str(event)?;
        Ok(event)
    }
}

pub async fn events_handler(events: Vec<Event<ManageEntity>>) -> AppResult {
    // TODO: make this atomic
    for event in events {
        handle_event(event).await?;
    }
    Ok(())
}

async fn handle_event(event: Event<ManageEntity>) -> AppResult {
    let event: Events = event.try_into()?;
    info!("{:#?}", event);
    Ok(())
}
