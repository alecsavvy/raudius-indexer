//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use super::sea_orm_active_enums::DelistTrackReason;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "track_delist_statuses")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key, auto_increment = false)]
    pub track_id: i32,
    pub owner_id: i32,
    pub track_cid: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub delisted: bool,
    pub reason: DelistTrackReason,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
