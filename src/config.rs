use serde::Deserialize;

pub type AppResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub nethermind_rpc: String,
    pub entity_manager_address: String,
    pub database_url: String,
    pub server_addr: String,
}
