use crate::{
    db::Repository, entity_manager::EntityManager, event_handler::handle_event, AppResult, Config,
};
use ethcontract::prelude::*;
use futures::StreamExt;
use std::str::FromStr;
use tracing::*;

pub async fn start(conf: Config, repo: Repository) -> AppResult {
    let Config {
        rpc_gateway,
        entity_manager_address,
        ..
    } = conf;
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
        // cheap clone
        let repo = repo.clone();
        // TODO: execution errors are swallowed here
        if let Some(Ok(event)) = events.next().await {
            if let Err(e) = handle_event(repo, event).await {
                error!("{:#?}", e);
            };
        }
    }
}
