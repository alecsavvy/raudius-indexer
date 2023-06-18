use serde::Deserialize;
use tracing_subscriber;

pub mod entity_manager;
pub mod index_blocks;
pub mod server;

pub type AppResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub nethermind_rpc: String,
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
    let Config { .. } = conf.clone();

    Ok(())
}
