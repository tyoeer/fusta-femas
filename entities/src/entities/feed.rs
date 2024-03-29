#[cfg(feature = "orm")]
use sea_orm::entity::prelude::*;
use ff_macros::{Object, FieldList};
use serde::{Deserialize, Serialize};

use crate::time_fields as time;


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Object, FieldList, bevy_reflect::Reflect)]
#[reflect(from_reflect = false)]
#[cfg_attr(feature="orm", derive(DeriveEntityModel) )]
#[cfg_attr(feature="orm", sea_orm(table_name = "feed") )]
pub struct Model {
	pub url: String,
	pub name: String,
	pub strategy: String,
	#[cfg_attr(feature="orm", sea_orm(primary_key) )]
	pub id: i32,
	pub created_at: time::PrimitiveDateTime,
	pub updated_at: time::PrimitiveDateTime,
}


pub type Ref = ff_object::ObjRef<Model>;


cfg_if::cfg_if! { if #[cfg(feature = "orm")] {


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::entry::Entity")]
	Entry,
	#[sea_orm(has_many = "super::fetch::Entity")]
	Fetch,
	#[sea_orm(has_many = "super::feed_tag::Entity")]
	FeedTag,
}

impl Related<super::entry::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Entry.def()
	}
}

impl Related<super::fetch::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Fetch.def()
	}
}

impl Related<super::feed_tag::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::FeedTag.def()
	}
}

impl Related<super::tag::Entity> for Entity {
	fn to() -> RelationDef {
		super::feed_tag::Relation::Tag.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::feed_tag::Relation::Feed.def().rev())
	}
}

impl ActiveModelBehavior for ActiveModel {}


}}
