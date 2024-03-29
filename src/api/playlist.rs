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
pub struct Playlist {
    #[serde(rename = "artwork", skip_serializing_if = "Option::is_none")]
    pub artwork: Option<Box<crate::api::PlaylistArtwork>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "permalink", skip_serializing_if = "Option::is_none")]
    pub permalink: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "is_album")]
    pub is_album: bool,
    #[serde(rename = "playlist_name")]
    pub playlist_name: String,
    #[serde(rename = "repost_count")]
    pub repost_count: i32,
    #[serde(rename = "favorite_count")]
    pub favorite_count: i32,
    #[serde(rename = "total_play_count")]
    pub total_play_count: i32,
    #[serde(rename = "user")]
    pub user: Box<crate::api::User>,
}

impl Playlist {
    pub fn new(
        id: String,
        is_album: bool,
        playlist_name: String,
        repost_count: i32,
        favorite_count: i32,
        total_play_count: i32,
        user: crate::api::User,
    ) -> Playlist {
        Playlist {
            artwork: None,
            description: None,
            permalink: None,
            id,
            is_album,
            playlist_name,
            repost_count,
            favorite_count,
            total_play_count,
            user: Box::new(user),
        }
    }
}
