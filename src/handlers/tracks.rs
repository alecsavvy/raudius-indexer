use crate::{actions::Track, db::TrackRepository, AppResult};

pub async fn create_track(repo: TrackRepository, track: Track) -> AppResult {
    repo.insert(track).await?;
    Ok(())
}
