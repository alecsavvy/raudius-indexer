use crate::{actions::User, api::UserResponse, db::users::UserRepository, AppResult};
/*
   REST ROUTES
*/
pub async fn get_user() -> AppResult<UserResponse> {
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
