// use leptos::*;
// use leptos_router::A;
use entities::*;
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
impl FetchOverview {
	fn base_query() -> sea_orm::Select<fetch::Entity> {
		fetch::Entity::find()
			.select_only()
			.columns(fetch::Column::iter().filter(|column| {
				use fetch::Column::*;
				!matches!(column, Content | Error | Log )
			}))
	}
	pub fn query(modifier: impl FnOnce(Select<fetch::Entity>) -> Select<fetch::Entity>)-> sea_orm::Selector<SelectModel<Self>> {
		modifier(Self::base_query())
			.into_model::<Self>()
	}
}