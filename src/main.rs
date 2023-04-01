use mongodb::{options::ClientOptions, Client};
use serde::Deserialize;
use std::error::Error;
use tokio::join;
use tracing_subscriber;

mod entity_manager;

mod actions;
#[allow(dead_code)] // codegen
mod api;
#[allow(dead_code)] // experimental
mod db;
mod event_handler;
mod handlers;
mod indexer;
mod routes;
mod server;

use crate::db::Repository;

pub type AppResult<T = ()> = Result<T, Box<dyn Error>>;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub rpc_gateway: String,
    pub entity_manager_address: String,
    pub db_connection_string: String,
    pub server_addr: String,
}

#[tokio::main]
async fn main() -> AppResult {
    use dotenv;
    dotenv::dotenv()?;

    tracing_subscriber::fmt::init();

    let conf = envy::from_env::<Config>()?;
    let Config {
        db_connection_string,
        ..
    } = conf.clone();

    let mut mongo_options = ClientOptions::parse(db_connection_string.as_str()).await?;
    mongo_options.app_name = Some("raudius-indexer".to_string());
    let mongo = Client::with_options(mongo_options)?;
    let db = mongo.database("audius-discovery");

    let repo = Repository::new(db);

    let server_conf = conf.clone();
    let server_repo = repo.clone();

    let web_server = tokio::spawn(async move {
        server::start(server_conf, server_repo)
            .await
            .expect("server crashed")
    });

    let indexer_conf = conf.clone();
    let indexer_repo = repo.clone();
    let acdc_indexer = tokio::spawn(async move {
        indexer::start(indexer_conf, indexer_repo)
            .await
            .expect("indexer crashed")
    });

    let (web_server, acdc_indexer) = join!(web_server, acdc_indexer);
    web_server?;
    acdc_indexer?;

    Ok(())
}
