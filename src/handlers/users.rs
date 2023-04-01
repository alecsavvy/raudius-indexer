use axum::{extract::Path, Extension, Json};

use crate::{
    actions::User,
    api::{User as InnerUserResponse, UserResponse},
    db::{users::UserRepository, Repository},
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
        id: user.user_id,
        ..Default::default()
    }));
    let res = Json(UserResponse { data });
    Ok(res)
}

/*
   ENTITY MANAGER ROUTES
*/
pub async fn create_user(repo: &UserRepository, user: User) -> AppResult {
    repo.insert(user).await?;
    Ok(())
}

pub async fn _update_user() -> AppResult {
    Ok(())
}
