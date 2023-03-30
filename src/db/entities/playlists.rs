use mongodb::bson::DateTime;

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub blockhash: Option<String>,
    pub blocknumber: Option<i32>,
    pub playlist_id: i32,
    pub playlist_owner_id: i32,
    pub is_album: bool,
    pub is_private: bool,
    pub playlist_name: Option<String>,
    // pub playlist_contents: Json,
    pub playlist_image_multihash: Option<String>,
    pub is_current: bool,
    pub is_delete: bool,
    pub description: Option<String>,
    pub created_at: DateTime,
    pub upc: Option<String>,
    pub updated_at: DateTime,
    pub playlist_image_sizes_multihash: Option<String>,
    pub txhash: String,
    pub last_added_to: Option<DateTime>,
    pub slot: Option<i32>,
    pub metadata_multihash: Option<String>,
}
