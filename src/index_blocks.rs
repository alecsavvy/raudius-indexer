use tracing::error;

use crate::{AppResult, process_block::process};

/// reads the latest block, checks db if it's been read
/// and passes to entity manager parser for side effects
pub async fn index_block() -> AppResult {
    // parse events in block and process
    process().await?;
    Ok(())
}

/// reads and sets configuration for how often blocks
/// get indexed
pub async fn index() -> AppResult {
    loop {
        // get latest block
        // check if read already in db
        if let Err(e) = index_block().await {
            error!(e, "error with indexing block")
        }
    }
}
