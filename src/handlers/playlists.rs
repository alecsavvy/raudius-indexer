use crate::{
    actions::Playlist, api::PlaylistResponse, db::playlists::PlaylistRepository, AppResult,
};
/*
   REST ROUTES
*/
pub async fn get_playlist() -> AppResult<PlaylistResponse> {
    Ok(PlaylistResponse::default())
}

/*
   ENTITY MANAGER ROUTES
*/
pub async fn create_playlist(repo: &PlaylistRepository, playlist: Playlist) -> AppResult {
    repo.insert(playlist).await?;
    Ok(())
}

pub async fn update_playlist() -> AppResult {
    Ok(())
}
