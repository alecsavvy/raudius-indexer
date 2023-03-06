use std::{error::Error, str::FromStr};

use ethcontract::Event;
use strum::EnumString;
use tracing::info;

use crate::{entity_manager::entity_manager::event_data::ManageEntity, AppResult};

#[derive(Debug, EnumString)]
pub enum Action {
    Create(CreateTrack),
    Update,
    // Delete,
    Follow,
    // Unfollow,
    Save,
    // Unsave,
    // Repost,
    // Unrepost,
    // Verify,
    // Subscribe,
    // Unsubscribe,
    // View,
    // ViewPlaylist,
    Default,
}

#[derive(Debug, Default)]
pub struct CreateTrack {
    pub track_id: String,
    pub owner_id: String,
    pub track_metadata: String,
}

impl TryFrom<Event<ManageEntity>> for Action {
    type Error = Box<dyn Error>;

    fn try_from(event: Event<ManageEntity>) -> AppResult<Self> {
        let action = Action::from_str(event.data.action.as_str())?;
        Ok(match action {
            Action::Create(_) => Action::Create(CreateTrack {
                track_id: event.data.entity_id.to_string(),
                owner_id: event.data.user_id.to_string(),
                track_metadata: event.data.metadata, // jsonify this i think?
            }),
            _ => action,
        })
    }
}

pub async fn events_handler(events: Vec<Event<ManageEntity>>) -> AppResult {
    for event in events {
        handle_event(event).await?;
    }
    Ok(())
}

async fn handle_event(event: Event<ManageEntity>) -> AppResult {
    let event: Action = event.try_into()?;
    info!("{:#?}", event);
    Ok(())
}
