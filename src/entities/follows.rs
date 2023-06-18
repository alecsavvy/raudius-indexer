//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "follows")]
pub struct Model {
    pub blockhash: Option<String>,
    pub blocknumber: Option<i32>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub follower_user_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub followee_user_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub is_current: bool,
    pub is_delete: bool,
    pub created_at: DateTime,
    #[sea_orm(primary_key, auto_increment = false)]
    pub txhash: String,
    pub slot: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
