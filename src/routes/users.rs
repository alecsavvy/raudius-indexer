use axum::{routing::get, Router};

pub fn routes(app: Router) -> Router {
    let user_routes = Router::new().route("/:user_id", get(mock_route));
    app.nest("/users", user_routes)
}

pub async fn mock_route() -> &'static str {
    "user route"
}
