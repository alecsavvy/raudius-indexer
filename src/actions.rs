use std::collections::HashSet;

use serde::{de, Deserialize, Serialize};

// // Typesafe variants of ManageEntity events
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "command")]
pub enum Actions {
    // UserReplicaSet Actions
    UpdateUserReplicaSet(UserReplicaSet),
    // User Actions
    CreateUser(User),
    UpdateUser(User),
    FollowUser(User),
    UnfollowUser(User),
    SubscribeUser(User),
    UnsubscribeUser(User),
    // Track Actions
    CreateTrack(Track),
    UpdateTrack(Track),
    RepostTrack(Track),
    UnrepostTrack(Track),
    SaveTrack(Track),
    UnsaveTrack(Track),
    DeleteTrack(Track),
    // Playlist Actions
    SavePlaylist(Playlist),
    UnsavePlaylist(Playlist),
    RepostPlaylist(Playlist),
    UnrepostPlaylist(Playlist),
    // Notification Actions
    ViewNotification(Notification),
    ViewPlaylistNotification(Notification),
}

// /*
// ENTITIES, TODO MOVE THESE
//  */
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub entity_id: String,
    pub metadata: String,
    pub signer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub user_id: String,
    pub entity_id: String,
    pub metadata: String,
    pub signer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub user_id: String,
    pub entity_id: String,
    #[serde(alias = "metadata")]
    pub cid: String,
    pub signer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    pub user_id: String,
    pub entity_id: String,
    pub metadata: String,
    pub signer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserReplicaSet {
    pub user_id: String,
    pub entity_id: String,
    // pub from: HashSet<u32>,
    #[serde(deserialize_with = "de_replica_set")]
    pub metadata: ReplicaSet,
    pub signer: String,
}

#[derive(Debug, Serialize)]
pub struct ReplicaSet {
    pub to: HashSet<String>,
    pub from: HashSet<String>,
}

fn de_replica_set<'de, D>(deserializer: D) -> Result<ReplicaSet, D::Error>
where
    D: de::Deserializer<'de>,
{
    let raw: String = de::Deserialize::deserialize(deserializer)?;
    let sets = raw.split(":").collect::<Vec<&str>>();
    let from = sets[0].split(",").map(|s| s.to_string()).collect();
    let to = sets[1].split(",").map(|s| s.to_string()).collect();
    Ok(ReplicaSet { to, from })
}
