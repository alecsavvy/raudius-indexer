use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Model {
    pub blockhash: Option<String>,

    pub user_id: i32,

    pub is_current: bool,
    pub handle: Option<String>,
    pub wallet: Option<String>,
    pub name: Option<String>,
    pub profile_picture: Option<String>,
    pub cover_photo: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub metadata_multihash: Option<String>,
    pub creator_node_endpoint: Option<String>,
    pub blocknumber: Option<i32>,
    pub is_verified: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub handle_lc: Option<String>,
    pub cover_photo_sizes: Option<String>,
    pub profile_picture_sizes: Option<String>,
    pub primary_id: Option<i32>,
    pub secondary_ids: Option<Vec<String>>,
    pub replica_set_update_signer: Option<String>,
    pub has_collectibles: bool,
    pub txhash: String,
    pub playlist_library: Option<String>,
    pub is_deactivated: bool,
    pub slot: Option<i32>,
    pub user_storage_account: Option<String>,
    pub user_authority_account: Option<String>,
    pub artist_pick_track_id: Option<i32>,
}
