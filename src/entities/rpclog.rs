//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "rpclog")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub cuid: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub wallet: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub method: Option<String>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub params: Option<Json>,
    pub jetstream_seq: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
