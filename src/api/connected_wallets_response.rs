/*
 * API
 *
 * Audius V1 API
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */
use fake::{Dummy, Fake};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, Dummy)]
pub struct ConnectedWalletsResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::api::ConnectedWallets>>,
}

impl ConnectedWalletsResponse {
    pub fn new() -> ConnectedWalletsResponse {
        ConnectedWalletsResponse { data: None }
    }
}
