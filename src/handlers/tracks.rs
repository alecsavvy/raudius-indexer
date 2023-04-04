use axum::{extract::Path, Extension};

use crate::{
    actions::Track,
    api::TrackResponse,
    db::{tracks::TrackRepository, Repository},
    AppResult,
};
/*
   REST ROUTES
*/
pub async fn get_track(
    Extension(repo): Extension<Repository>,
    Path(track_id): Path<String>,
) -> AppResult<TrackResponse> {
    let track = repo.tracks.get(track_id.as_str()).await?;
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
