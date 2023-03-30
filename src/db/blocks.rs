use mongodb::{bson::doc, error::Error, options::FindOptions, Collection, Database};

use super::entities::blocks::BlockModel;

#[derive(Debug, Clone)]
pub struct BlockRepository {
    collection: Collection<BlockModel>,
}

impl BlockRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("Blocks");
        Self { collection }
    }

    pub async fn insert(
        &self,
        blockhash: String,
        number: i32,
        parenthash: Option<String>,
    ) -> Result<(), Error> {
        let block = BlockModel {
            blockhash,
            number,
            parenthash,
        };
        self.collection.insert_one(block, None).await?;
        Ok(())
    }

    pub async fn get_latest(&self) -> Result<BlockModel, Error> {
        let sort = doc! {"_id": -1 };
        let block = self
            .collection
            .find(
                None,
                Some(FindOptions::builder().sort(sort).limit(Some(1)).build()),
            )
            .await?
            .deserialize_current()?;
        Ok(block)
    }
}
