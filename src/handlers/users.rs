use crate::{api::UserResponse, AppResult};
/*
   REST ROUTES
*/
pub async fn get_user() -> AppResult<UserResponse> {
    Ok(UserResponse::default())
}

/*
   ENTITY MANAGER ROUTES
*/
pub async fn create_user() -> AppResult {
    Ok(())
}

pub async fn update_user() -> AppResult {
    Ok(())
}
