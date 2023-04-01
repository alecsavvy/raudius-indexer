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
pub struct RemixParent {
    #[serde(rename = "tracks", skip_serializing_if = "Option::is_none")]
    pub tracks: Option<Vec<crate::api::TrackElement>>,
}

impl RemixParent {
    pub fn new() -> RemixParent {
        RemixParent { tracks: None }
    }
}
