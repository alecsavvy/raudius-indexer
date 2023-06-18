//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "aggregate_playlist")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub playlist_id: i32,
    pub is_album: Option<bool>,
    pub repost_count: Option<i32>,
    pub save_count: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
