use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::AppResult;

pub async fn fetch_cid_metadata<T>(cid: &str) -> AppResult<T>
where
    T: DeserializeOwned,
{
    // 1 get replica set for user

    // 2 fetch all content nodes

    // 3 query primary content node

    // 4 if error, query secondary nodes
    todo!()
}
