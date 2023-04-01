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
pub struct TrackArtwork {
    #[serde(rename = "150x150", skip_serializing_if = "Option::is_none")]
    pub param_150x150: Option<String>,
    #[serde(rename = "480x480", skip_serializing_if = "Option::is_none")]
    pub param_480x480: Option<String>,
    #[serde(rename = "1000x1000", skip_serializing_if = "Option::is_none")]
    pub param_1000x1000: Option<String>,
}

impl TrackArtwork {
    pub fn new() -> TrackArtwork {
        TrackArtwork {
            param_150x150: None,
            param_480x480: None,
            param_1000x1000: None,
        }
    }
}
