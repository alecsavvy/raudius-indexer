//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "trending_results")]
pub struct Model {
    pub user_id: i32,
    pub id: Option<String>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub rank: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub r#type: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub version: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub week: Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}