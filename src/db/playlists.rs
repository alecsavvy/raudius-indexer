use mongodb::{bson::doc, error::Error, Collection, Database};

use crate::actions::Playlist;

use super::entities::playlists::Model;

#[derive(Debug, Clone)]
pub struct PlaylistRepository {
    collection: Collection<Model>,
}

impl PlaylistRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("playlists");
        Self { collection }
    }

    pub async fn insert(&self, playlist: Playlist) -> Result<Playlist, Error> {
        tracing::info!("meta {}", playlist.metadata);
        // let model = Model {
        //     blockhash: todo!(),
        //     blocknumber: todo!(),
        //     playlist_id: todo!(),
        //     playlist_owner_id: todo!(),
        //     is_album: todo!(),
        //     is_private: todo!(),
        //     playlist_name: todo!(),
        //     playlist_image_multihash: todo!(),
        //     is_current: todo!(),
        //     is_delete: todo!(),
        //     description: todo!(),
        //     created_at: todo!(),
        //     upc: todo!(),
        //     updated_at: todo!(),
        //     playlist_image_sizes_multihash: todo!(),
        //     txhash: todo!(),
        //     last_added_to: todo!(),
        //     slot: todo!(),
        //     metadata_multihash: todo!(),
        // };
        // self.collection.insert_one(model, None).await?;
        Ok(playlist)
    }

    /// uses entity id
    pub async fn get(&self, entity_id: &str) -> Result<Model, Error> {
        let playlist = self
            .collection
            .find_one(doc! {"entity_id": entity_id}, None)
            .await?;
        Ok(playlist.unwrap())
    }
}
