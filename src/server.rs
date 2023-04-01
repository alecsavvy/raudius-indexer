use std::net::SocketAddr;

use axum::{routing::get, Router};

use crate::{db::Repository, AppResult, Config};

pub async fn start(conf: Config, _repo: Repository) -> AppResult {
    let app = Router::new().route("/", get(root));

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
