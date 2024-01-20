#[cfg(feature = "orm")]
use sea_orm::entity::prelude::*;
use ff_macros::{Object, FieldList};
use serde::{Deserialize, Serialize};

use crate::time_fields as time;


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Object, FieldList, bevy_reflect::Reflect)]
#[reflect(from_reflect = false)]
#[cfg_attr(feature="orm", derive(DeriveEntityModel) )]
#[cfg_attr(feature="orm", sea_orm(table_name = "feed_tag") )]
pub struct Model {
	pub feed_id: i32,
	pub tag_id: i32,
	#[cfg_attr(feature="orm", sea_orm(primary_key) )]
	pub id: i32,
	pub created_at: time::PrimitiveDateTime,
	pub updated_at: time::PrimitiveDateTime,
}


pub type Ref = ff_object::ObjRef<Model>;


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
	#[sea_orm(
		belongs_to = "super::tag::Entity",
		from = "Column::TagId",
		to = "super::tag::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Tag,
}

impl Related<super::feed::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Feed.def()
	}
}

impl Related<super::tag::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Tag.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}


} }