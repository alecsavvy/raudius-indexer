use axum::{extract::Path, Extension, Json};
use hash_ids::HashIds;

use crate::{
    actions::User,
    api::{User as InnerUserResponse, UserResponse},
    db::{entities::users::Model, users::UserRepository, Repository},
    error::AppError,
    AppResult,
};
/*
   REST ROUTES
*/
pub async fn get_user(
    Extension(repo): Extension<Repository>,
    Path(user_id): Path<String>,
) -> AppResult<Json<UserResponse>> {
    let user = repo.users.get(user_id.as_str()).await?;
    let user = user.ok_or_else(|| AppError::UserNotFound(user_id))?;
    let data = Some(Box::new(InnerUserResponse {
        id: user.user_id.to_string(),
        ..Default::default()
    }));
    let res = Json(UserResponse { data });
    Ok(res)
}

/*
   ENTITY MANAGER ROUTES
*/
pub async fn create_user(repo: &UserRepository, user: User) -> AppResult {
    // let user_id = hasher.decode(&user.user_id); // returns slice?
    let record = Model {
        blockhash: todo!(),
        user_id: todo!(),
        is_current: todo!(),
        handle: todo!(),
        wallet: todo!(),
        name: todo!(),
        profile_picture: todo!(),
        cover_photo: todo!(),
        bio: todo!(),
        location: todo!(),
        metadata_multihash: todo!(),
        creator_node_endpoint: todo!(),
        blocknumber: todo!(),
        is_verified: todo!(),
        created_at: todo!(),
        updated_at: todo!(),
        handle_lc: todo!(),
        cover_photo_sizes: todo!(),
        profile_picture_sizes: todo!(),
        primary_id: todo!(),
        secondary_ids: todo!(),
        replica_set_update_signer: todo!(),
        has_collectibles: todo!(),
        txhash: todo!(),
        playlist_library: todo!(),
        is_deactivated: todo!(),
        slot: todo!(),
        user_storage_account: todo!(),
        user_authority_account: todo!(),
        artist_pick_track_id: todo!(),
    };
    repo.insert(record).await?;
    Ok(())
}

pub async fn _update_user() -> AppResult {
    Ok(())
}
