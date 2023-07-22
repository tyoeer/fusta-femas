//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "entry")]
pub struct Model {
	pub name: String,
	pub view_url: String,
	pub embed_url: Option<String>,
	pub viewed: bool,
	pub feed_entry_id: String,
	pub feed_id: i32,
	pub latest_fetch_id: Option<i32>,
	pub date: Option<TimeDateTime>,
	#[sea_orm(primary_key)]
	pub id: i32,
	pub created_at: TimeDateTime,
	pub updated_at: TimeDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::feed::Entity",
		from = "Column::FeedId",
		to = "super::feed::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Feed,
	#[sea_orm(
		belongs_to = "super::fetch::Entity",
		from = "Column::LatestFetchId",
		to = "super::fetch::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Fetch,
}

impl Related<super::feed::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Feed.def()
	}
}

impl Related<super::fetch::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Fetch.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
