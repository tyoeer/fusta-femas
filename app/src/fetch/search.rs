// use leptos::*;
// use leptos_router::A;
use entities::prelude::*;
// use crate::table;
// use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;
#[cfg(feature="ssr")]
use ff_object::View;


#[derive(
	Clone, Debug, PartialEq, Eq,
	serde::Serialize, serde::Deserialize,
	bevy_reflect::Reflect
)]
#[cfg_attr(feature="ssr", derive(FromQueryResult))]
#[reflect(from_reflect = false)]
pub struct FetchOverview {
	pub id: i32,
	pub url: String,
	pub status: fetch::Status,
	// pub content: Option<String>,
	// pub error: Option<String>,
	// pub log: String,
	pub strategy: String,
	pub feed_id: i32,
	pub created_at: time_fields::PrimitiveDateTime,
	pub updated_at: time_fields::PrimitiveDateTime,
}


#[cfg(feature="ssr")]
impl View for FetchOverview {
	type Entity = fetch::Entity;
	
	fn columns() -> impl Iterator<Item = impl sea_orm::ColumnTrait> {
		fetch::Column::iter().filter(|column| {
			use fetch::Column::*;
			!matches!(column, Content | Error | Log )
		})
	}
}

impl ff_object::Object for FetchOverview {
	fn get_id(&self) -> i32 {
		self.id
	}
	
	fn get_object_name() -> &'static str {
		"fetch"
	}
}