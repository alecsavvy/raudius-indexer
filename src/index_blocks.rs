use crate::AppResult;

/// reads the latest block, checks db if it's been read
/// and passes to entity manager parser for side effects
pub async fn index_block() -> AppResult {
    Ok(())
}

/// reads and sets configuration for how often blocks
/// get indexed
pub async fn index() -> AppResult {
    Ok(())
}
