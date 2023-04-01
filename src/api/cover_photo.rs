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
pub struct CoverPhoto {
    #[serde(rename = "640x", skip_serializing_if = "Option::is_none")]
    pub param_640x: Option<String>,
    #[serde(rename = "2000x", skip_serializing_if = "Option::is_none")]
    pub param_2000x: Option<String>,
}

impl CoverPhoto {
    pub fn new() -> CoverPhoto {
        CoverPhoto {
            param_640x: None,
            param_2000x: None,
        }
    }
}
