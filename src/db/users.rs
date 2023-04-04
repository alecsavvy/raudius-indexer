use crate::db::entities::users::Model as UserModel;
use mongodb::{bson::doc, error::Error, Collection, Database};

#[derive(Debug, Clone)]
pub struct UserRepository {
    collection: Collection<UserModel>,
}

impl UserRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("users");
        Self { collection }
    }

    pub async fn insert(&self, user: UserModel) -> Result<UserModel, Error> {
        self.collection.insert_one(user.clone(), None).await?;
        Ok(user)
    }

    /// uses entity id
    pub async fn get(&self, entity_id: &str) -> Result<Option<UserModel>, Error> {
        let user = self
            .collection
            .find_one(doc! {"user_id": entity_id}, None)
            .await?;
        Ok(user)
    }
}
