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
pub struct UserSubscribers {
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "subscriber_ids", skip_serializing_if = "Option::is_none")]
    pub subscriber_ids: Option<Vec<String>>,
}

impl UserSubscribers {
    pub fn new(user_id: String) -> UserSubscribers {
        UserSubscribers {
            user_id,
            subscriber_ids: None,
        }
    }
}
