use mongodb::bson::DateTime;

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub blockhash: Option<String>,
    pub blocknumber: Option<i32>,
    pub follower_user_id: i32,
    pub followee_user_id: i32,
    pub is_current: bool,
    pub is_delete: bool,
    pub created_at: DateTime,
    pub txhash: String,
    pub slot: Option<i32>,
}
