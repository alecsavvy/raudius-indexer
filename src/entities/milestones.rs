//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "milestones")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub name: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub threshold: i32,
    pub blocknumber: Option<i32>,
    pub slot: Option<i32>,
    pub timestamp: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
