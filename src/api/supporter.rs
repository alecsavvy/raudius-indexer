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
pub struct Supporter {
    #[serde(rename = "rank")]
    pub rank: i32,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "sender")]
    pub sender: Box<crate::api::User>,
}

impl Supporter {
    pub fn new(rank: i32, amount: String, sender: crate::api::User) -> Supporter {
        Supporter {
            rank,
            amount,
            sender: Box::new(sender),
        }
    }
}
