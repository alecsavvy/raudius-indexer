use mongodb::{bson::doc, error::Error, Collection, Database};

use crate::{actions::User, error::AppError};

#[derive(Debug, Clone)]
pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("users");
        Self { collection }
    }

    pub async fn insert(&self, user: User) -> Result<User, Error> {
        self.collection.insert_one(user.clone(), None).await?;
        Ok(user)
    }

    /// uses entity id
    pub async fn get(&self, entity_id: &str) -> Result<Option<User>, Error> {
        let user = self
            .collection
            .find_one(doc! {"entity_id": entity_id}, None)
            .await?;
        Ok(user)
    }
}
