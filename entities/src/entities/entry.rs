#[cfg(feature = "orm")]
use sea_orm::entity::prelude::*;
use ff_macros::Object;
use serde::{Deserialize, Serialize};

use crate::time_fields as time;


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Object, bevy_reflect::Reflect)]
#[reflect(from_reflect = false)]
#[cfg_attr(feature="orm", derive(DeriveEntityModel) )]
#[cfg_attr(feature="orm", sea_orm(table_name = "entry") )]
pub struct Model {
	pub name: String,
	///The natural page containing the content
	pub view_url: String,
	///Just the content
	pub embed_url: Option<String>,
	pub viewed: bool,
	pub feed_entry_id: String,
	pub feed_id: i32,
	pub produced_date: time::Date,
	pub produced_time: time::OptionTime,
	#[cfg_attr(feature="orm", sea_orm(primary_key) )]
	pub id: i32,
	pub created_at: time::PrimitiveDateTime,
	pub updated_at: time::PrimitiveDateTime,
}


cfg_if::cfg_if! { if #[cfg(feature = "orm")] {

	
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
	// #[sea_orm(has_many = "super::fetch::Entity")]
	// Fetch,
	#[sea_orm(has_many = "super::fetch_entry::Entity")]
	FetchEntry,
}

impl Related<super::feed::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Feed.def()
	}
}

impl Related<super::fetch::Entity> for Entity {
	fn to() -> RelationDef {
		super::fetch_entry::Relation::Fetch.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::fetch_entry::Relation::Entry.def().rev())
	}
}

impl Related<super::fetch_entry::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::FetchEntry.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

}}
