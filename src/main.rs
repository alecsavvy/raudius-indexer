use ethcontract::prelude::*;
use futures::StreamExt;
use serde::Deserialize;
use std::{error::Error, str::FromStr};

mod entity_manager;
use entity_manager::EntityManager;

mod actions;
mod event_handler;

use tracing::*;
use tracing_subscriber;

use crate::event_handler::handle_event;

pub type AppResult<T = ()> = Result<T, Box<dyn Error>>;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rpc_gateway: String,
    pub entity_manager_address: String,
}

#[tokio::main]
async fn main() -> AppResult {
    use dotenv;
    dotenv::dotenv()?;

    tracing_subscriber::fmt::init();

    let Config {
        rpc_gateway,
        entity_manager_address,
    } = envy::from_env::<Config>()?;

    let http = Http::new(&rpc_gateway)?;
    let web3 = Web3::new(http);

    let em_address = Address::from_str(&entity_manager_address)?;
    let em_contract = EntityManager::with_deployment_info(&web3, em_address, None);

    // log current block
    let current_block = web3.eth().block_number().await?;
    info!(
        "Entity Manager Contract: {:#?}, Block Number: {}",
        em_contract, current_block
    );

    let events = em_contract
        .all_events()
        .from_block(BlockNumber::Earliest)
        .query_paginated()
        .await?;

    let mut events = Box::pin(events);

    loop {
        // TODO: execution errors are swallowed here
        if let Some(Ok(event)) = events.next().await {
            if let Err(e) = handle_event(event).await {
                error!("{:#?}", e);
            };
        }
    }
}
