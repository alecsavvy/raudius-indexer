use std::net::SocketAddr;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

use crate::{db::Repository, routes::users, AppResult, Config};

pub async fn start(conf: Config, _repo: Repository) -> AppResult {
    let app = Router::new().route("/", get(root));
    let app = users::routes(app);
    let app = app.fallback(not_found);

    let addr = conf.server_addr.parse::<SocketAddr>()?;
    tracing::debug!("starting server on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn root() -> &'static str {
    "howdy!"
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "you look lost!")
}
