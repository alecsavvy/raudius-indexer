use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockModel {
    pub blockhash: String,
    pub parenthash: Option<String>,
    pub number: i32,
}
