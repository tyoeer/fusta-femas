// use leptos::*;
// use leptos_router::A;
use entities::prelude::*;
// use crate::table;
// use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;

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
type Entity = fetch::Entity;

#[cfg(feature="ssr")]
impl FetchOverview {
	fn columns() -> impl Iterator<Item = impl sea_orm::ColumnTrait> {
		fetch::Column::iter().filter(|column| {
			use fetch::Column::*;
			!matches!(column, Content | Error | Log )
		})
	}
	
	fn order(query: Select<Entity>) -> Select<Entity> {
		query
	}
		
	
	pub fn query(modifier: impl FnOnce(Select<Entity>) -> Select<Entity>) -> sea_orm::Selector<SelectModel<Self>> {
		let query = Entity::find();
		let query = modifier(query);
		Self::from_query(query)
	}
	
	
	pub fn from_query(query: Select<Entity>) -> sea_orm::Selector<SelectModel<Self>> {
		let query = Self::order(query);
		let query = Self::select_only_columns(query);
		query.into_model::<Self>()
	}
	
	fn select_only_columns(query: Select<Entity>) -> Select<Entity> {
		query
			.select_only()
			.columns(Self::columns())
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