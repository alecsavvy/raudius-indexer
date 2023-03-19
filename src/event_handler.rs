use ethcontract::Event;
use tracing::info;

use crate::{
    actions::Actions,
    entity_manager::entity_manager::{
        event_data::{ManageEntity, ManageIsVerified},
        Event as EmEvent,
    },
    AppResult,
};

pub async fn handle_event(event: Event<EmEvent>) -> AppResult {
    match event.data {
        EmEvent::ManageEntity(me) => handle_entity(&me).await?,
        EmEvent::ManageIsVerified(miv) => handle_is_verified(&miv).await?,
    };
    Ok(())
}

/// Handles the conversion of a raw ManageEntity event
async fn handle_entity(event: &ManageEntity) -> AppResult {
    // 1. parse out kind of event we have for the cid type
    // 2. get the primary content node from that user
    // 3. attempt to gather the content from that node by the cid in event
    // 4. if failure gather content replicas and try with those
    // 5. serialize content into inner metadata structure based on event type
    // 6. construct AppEvents variant and return

    // UpdateUser for example
    let command = format!("{}{}", event.action, event.entity_type);
    let mut json = serde_json::to_value(event)?;

    json["command"] = serde_json::Value::String(command);

    // info!("{}", json);

    let action = serde_json::from_value::<Actions>(json)?;
    info!("{:?}", action);
    /*
    handle with SERDE
       if event_type == EntityType.PLAYLIST:
           cid_type[cid] = "playlist_data"
       elif event_type == EntityType.TRACK:
           cid_type[cid] = "track"
       elif event_type == EntityType.USER:
           cid_type[cid] = "user"
    */

    Ok(())
}

/// Handles the conversion of a raw ManageIsVerified event
async fn handle_is_verified(event: &ManageIsVerified) -> AppResult {
    info!("user {} is_verified {}", event.user_id, event.is_verified);
    Ok(())
}
