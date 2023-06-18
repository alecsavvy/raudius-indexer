//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "cid_data")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub cid: String,
    pub r#type: Option<String>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub data: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
