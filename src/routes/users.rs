use axum::{routing::get, Router};

pub fn routes(app: Router) -> Router {
    let user_routes = Router::new()
        .route("/:user_id", get(mock_route))
        .route("/id", get(mock_route))
        .route("/search", get(mock_route))
        .route("/:user_id/connected_wallets", get(mock_route))
        .route("/verify_token", get(mock_route))
        .route("/:user_id/favorites", get(mock_route))
        .route("/:user_id/reposts", get(mock_route))
        .route("/:user_id/supporters", get(mock_route))
        .route("/:user_id/supporting", get(mock_route))
        .route("/:user_id/tags", get(mock_route))
        .route("/:user_id/tracks", get(mock_route));
    app.nest("/users", user_routes)
}

pub async fn mock_route() -> &'static str {
    "user route"
}
