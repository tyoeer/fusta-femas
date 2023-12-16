#[cfg(feature = "orm")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::time_fields as time;
use crate::traits;


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, bevy_reflect::Reflect)]
#[reflect(from_reflect = false)]
#[cfg_attr(feature="orm", derive(DeriveEntityModel) )]
#[cfg_attr(feature="orm", sea_orm(table_name = "entry") )]
pub struct Model {
	pub name: String,
	pub view_url: String,
	pub embed_url: Option<String>,
	pub viewed: bool,
	pub feed_entry_id: String,
	pub feed_id: i32,
	pub latest_fetch_id: Option<i32>,
	pub produced_date: time::Date,
	pub produced_time: time::OptionTime,
	#[cfg_attr(feature="orm", sea_orm(primary_key) )]
	pub id: i32,
	pub created_at: time::PrimitiveDateTime,
	pub updated_at: time::PrimitiveDateTime,
}


impl traits::Object for Model {
	fn get_id(&self) -> i32 {
		self.id
	}
	
	fn get_object_name() -> &'static str {
		"entry"
	}
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

}}
