use ethcontract::prelude::*;
use std::{error::Error, str::FromStr};

mod entity_manager;
use entity_manager::EntityManager;

use tracing::*;
use tracing_subscriber;

use ethcontract::web3::types::U64;

// config constants for now
const RPC_GATEWAY: &'static str = "https://acdc-gateway.audius.co";
const ENTITY_MANAGER_ADDRESS: &'static str = "0x1Cd8a543596D499B9b6E7a6eC15ECd2B7857Fd64";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let http = Http::new(RPC_GATEWAY)?;
    let web3 = Web3::new(http);

    let em_address = Address::from_str(ENTITY_MANAGER_ADDRESS)?;
    let em_contract = EntityManager::with_deployment_info(&web3, em_address, None);

    let current_block = web3.eth().block_number().await?;

    info!(
        "Entity Manager Contract: {:#?}, Block Number: {}",
        em_contract, current_block
    );
    let events = em_contract
        .events()
        .manage_entity()
        .from_block(BlockNumber::Number(U64::from(500000)))
        .to_block(BlockNumber::Number(U64::from(500020)))
        .query()
        .await?;

    for event in events {
        info!("event: {:?}", event);
    }

    Ok(())
}
