#[cfg(feature = "orm")]
use sea_orm::entity::prelude::*;
use ff_macros::{Object, FieldList};
use serde::{Deserialize, Serialize};

use crate::time_fields as time;


#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, bevy_reflect::Reflect)]
#[reflect(from_reflect = false)]
#[cfg_attr(feature="orm", derive(EnumIter, DeriveActiveEnum) )]
#[cfg_attr(feature="orm", sea_orm(rs_type = "String", db_type = "String(Some(20))") )]
pub enum Status {
	#[cfg_attr(feature="orm", sea_orm(string_value = "SUCCESS") )]
	Success,
	#[cfg_attr(feature="orm", sea_orm(string_value = "FETCH_ERROR") )]
	FetchError,
	#[cfg_attr(feature="orm", sea_orm(string_value = "PARSE_ERROR") )]
	ParseError,
	#[cfg_attr(feature="orm", sea_orm(string_value = "ENTRY_UPDATE_ERROR") )]
	EntryUpdateError,
}

impl std::fmt::Display for Status {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let str = match self {
			Self::Success => "✅ Success",
			Self::FetchError => "❌ Error Fetching",
			Self::ParseError => "❌ Error Parsing",
			Self::EntryUpdateError => "❌ Error Updating Entries",
		};
		write!(f, "{str}")
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Object, FieldList, bevy_reflect::Reflect)]
#[reflect(from_reflect = false)]
#[cfg_attr(feature="orm", derive(DeriveEntityModel) )]
#[cfg_attr(feature="orm", sea_orm(table_name = "fetch") )]
pub struct Model {
	pub url: String,
	pub status: Status,
	pub content: Option<String>,
	pub error: Option<String>,
	pub log: String,
	pub strategy: String,
	pub feed_id: i32,
	#[cfg_attr(feature="orm", sea_orm(primary_key) )]
	pub id: i32,
	pub created_at: time::PrimitiveDateTime,
	pub updated_at: time::PrimitiveDateTime,
}


cfg_if::cfg_if! { if #[cfg(feature = "orm")] {


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	// #[sea_orm(has_many = "super::entry::Entity")]
	// Entry,
	#[sea_orm(has_many = "super::fetch_entry::Entity")]
	FetchEntry,
	#[sea_orm(
		belongs_to = "super::feed::Entity",
		from = "Column::FeedId",
		to = "super::feed::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Feed,
}

impl Related<super::entry::Entity> for Entity {
	fn to() -> RelationDef {
		super::fetch_entry::Relation::Entry.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::fetch_entry::Relation::Fetch.def().rev())
	}
}

impl Related<super::feed::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Feed.def()
	}
}

impl Related<super::fetch_entry::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::FetchEntry.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}


}}
