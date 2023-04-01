use crate::{api::PlaylistResponse, AppResult};
/*
   REST ROUTES
*/
pub async fn get_playlist() -> AppResult<PlaylistResponse> {
    Ok(PlaylistResponse::default())
}

/*
   ENTITY MANAGER ROUTES
*/
pub async fn create_playlist() -> AppResult {
    Ok(())
}

pub async fn update_playlist() -> AppResult {
    Ok(())
}
