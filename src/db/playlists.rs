use mongodb::{bson::doc, error::Error, Collection, Database};

use crate::actions::Playlist;

#[derive(Debug, Clone)]
pub struct PlaylistRepository {
    collection: Collection<Playlist>,
}

impl PlaylistRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("playlists");
        Self { collection }
    }

    pub async fn insert(&self, playlist: Playlist) -> Result<Playlist, Error> {
        self.collection.insert_one(playlist.clone(), None).await?;
        Ok(playlist)
    }

    /// uses entity id
    pub async fn get(&self, entity_id: &str) -> Result<Playlist, Error> {
        let playlist = self
            .collection
            .find_one(doc! {"entity_id": entity_id}, None)
            .await?;
        Ok(playlist.unwrap())
    }
}
