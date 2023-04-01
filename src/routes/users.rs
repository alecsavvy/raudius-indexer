use axum::{routing::get, Router};

use crate::handlers::users::get_user;

pub fn routes(app: Router) -> Router {
    let user_routes = Router::new().route("/:user_id", get(get_user));
    app.nest("/users", user_routes)
}
