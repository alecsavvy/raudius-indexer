use mongodb::bson::DateTime;

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub id: i32,
    pub user_id: Option<i32>,
    pub source: Option<String>,
    pub play_item_id: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub slot: Option<i32>,
    pub signature: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
}
