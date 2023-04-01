use axum::{extract::Path, Extension};

use crate::{
    actions::User,
    api::UserResponse,
    db::{users::UserRepository, Repository},
    AppResult,
};
/*
   REST ROUTES
*/
pub async fn get_user(
    Extension(repo): Extension<Repository>,
    Path(user_id): Path<String>,
) -> AppResult<UserResponse> {
    let _user = repo.users.get(user_id.as_str()).await?;
    Ok(UserResponse::default())
}

/*
   ENTITY MANAGER ROUTES
*/
pub async fn create_user(repo: &UserRepository, user: User) -> AppResult {
    repo.insert(user).await?;
    Ok(())
}

pub async fn update_user() -> AppResult {
    Ok(())
}
