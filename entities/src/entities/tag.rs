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


cfg_if::cfg_if! { if #[cfg(feature = "orm")] {


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	// #[sea_orm(has_many = "super::entry::Entity")]
	// Entry,
	// #[sea_orm(has_many = "super::fetch::Entity")]
	// Fetch,
}

// impl Related<super::entry::Entity> for Entity {
// 	fn to() -> RelationDef {
// 		Relation::Entry.def()
// 	}
// }

// impl Related<super::fetch::Entity> for Entity {
// 	fn to() -> RelationDef {
// 		Relation::Fetch.def()
// 	}
// }

impl ActiveModelBehavior for ActiveModel {}


}}
