use mongodb::{bson::doc, error::Error, Collection, Database};

use crate::actions::Track;

#[derive(Debug, Clone)]
pub struct TrackRepository {
    collection: Collection<Track>,
}

impl TrackRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("tracks");
        Self { collection }
    }

    pub async fn insert(&self, track: Track) -> Result<Track, Error> {
        self.collection.insert_one(track.clone(), None).await?;
        Ok(track)
    }

    /// uses entity id
    pub async fn get(&self, entity_id: &str) -> Result<Track, Error> {
        let track = self
            .collection
            .find_one(doc! {"entity_id": entity_id}, None)
            .await?;
        Ok(track.unwrap())
    }

    pub async fn get_by_cid(&self, cid: &str) -> Result<Track, Error> {
        let track = self.collection.find_one(doc! {"cid": cid}, None).await?;
        Ok(track.unwrap())
    }
}
