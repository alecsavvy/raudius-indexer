use config::AppResult;
use tracing_subscriber;

use crate::config::Config;

pub mod config;
pub mod error;
#[allow(unused_imports)]
pub mod generated;
pub mod server;

#[tokio::main]
async fn main() -> AppResult {
    use dotenv;
    dotenv::dotenv()?;

    tracing_subscriber::fmt::init();

    let _conf = envy::from_env::<Config>()?;

    Ok(())
}
