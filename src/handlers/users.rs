use axum::{extract::Path, Extension, Json};

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
) -> AppResult<Json<UserResponse>> {
    //let _user = repo.users.get(user_id.as_str()).await?;
    let res = Json(UserResponse::default());
    Ok(res)
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
