#[cfg(feature = "orm")]
use sea_orm::entity::prelude::*;
use ff_macros::Object;
use serde::{Deserialize, Serialize};

use crate::time_fields as time;


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Object, bevy_reflect::Reflect)]
#[reflect(from_reflect = false)]
#[cfg_attr(feature="orm", derive(DeriveEntityModel) )]
#[cfg_attr(feature="orm", sea_orm(table_name = "tag") )]
pub struct Model {
	pub title: String,
	#[cfg_attr(feature="orm", sea_orm(column_name="type") )]
	pub kind: String,
	pub config: Option<Vec<u8>>,
	#[cfg_attr(feature="orm", sea_orm(primary_key) )]
	pub id: i32,
	pub created_at: time::PrimitiveDateTime,
	pub updated_at: time::PrimitiveDateTime,
}

//TODO better way for this, probably using a newtype
pub fn types() -> impl Iterator<Item=String> {
	[
		"feed",
	].into_iter().map(|str| str.to_owned())
}

cfg_if::cfg_if! { if #[cfg(feature = "orm")] {


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::feed_tag::Entity")]
	FeedTag,
}

impl Related<super::feed_tag::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::FeedTag.def()
	}
}

impl Related<super::feed::Entity> for Entity {
	fn to() -> RelationDef {
		super::feed_tag::Relation::Feed.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::feed_tag::Relation::Tag.def().rev())
	}
}

impl ActiveModelBehavior for ActiveModel {}


}}
