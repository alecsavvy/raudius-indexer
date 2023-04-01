use crate::{actions::Track, api::TrackResponse, db::tracks::TrackRepository, AppResult};
/*
   REST ROUTES
*/
pub async fn get_track() -> AppResult<TrackResponse> {
    Ok(TrackResponse::default())
}

/*
   ENTITY MANAGER ROUTES
*/
pub async fn create_track(repo: &TrackRepository, track: Track) -> AppResult {
    repo.insert(track).await?;
    Ok(())
}

pub async fn _update_track() -> AppResult {
    Ok(())
}
