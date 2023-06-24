use axum::{routing::get, Router};

use crate::{AppResult, Config};

pub async fn web_server(config: Config) -> AppResult {
    let app = Router::new().route("/up", get(up));
    axum::Server::bind(&config.server_addr.parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn up() -> &'static str {
    "up"
}
