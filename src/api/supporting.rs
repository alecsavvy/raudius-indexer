/*
 * API
 *
 * Audius V1 API
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Supporting {
    #[serde(rename = "rank")]
    pub rank: i32,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "receiver")]
    pub receiver: Box<crate::api::User>,
}

impl Supporting {
    pub fn new(rank: i32, amount: String, receiver: crate::api::User) -> Supporting {
        Supporting {
            rank,
            amount,
            receiver: Box::new(receiver),
        }
    }
}
