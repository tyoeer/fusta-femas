#[cfg(feature = "orm")]
use sea_orm::entity::prelude::*;
use ff_macros::{Object, FieldList};
use serde::{Deserialize, Serialize};

use crate::time_fields as time;


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Object, FieldList, bevy_reflect::Reflect)]
#[reflect(from_reflect = false)]
#[cfg_attr(feature="orm", derive(DeriveEntityModel) )]
#[cfg_attr(feature="orm", sea_orm(table_name = "fetch_entry") )]
pub struct Model {
	pub entry_id: i32,
	pub fetch_id: i32,
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
		belongs_to = "super::entry::Entity",
		from = "Column::EntryId",
		to = "super::entry::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Entry,
	#[sea_orm(
		belongs_to = "super::fetch::Entity",
		from = "Column::FetchId",
		to = "super::fetch::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Fetch,
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

impl ActiveModelBehavior for ActiveModel {}


} }