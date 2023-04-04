use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::AppResult;

pub async fn fetch_cid_metadata<T>(cid: &str) -> AppResult<T>
where
    T: DeserializeOwned,
{
    todo!()
}
