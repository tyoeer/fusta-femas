use leptos::*;
// use leptos_router::A;
use entities::prelude::*;
use leptos_meta::Title;
use leptos_router::{Route, Outlet, A};
use crate::table;
use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;


#[derive(
	Clone, Debug, PartialEq, Eq,
	serde::Serialize, serde::Deserialize,
	bevy_reflect::Reflect
)]
#[cfg_attr(feature="ssr", derive(FromQueryResult))]
#[reflect(from_reflect = false)]
pub struct EntryOverview {
	pub name: String,
	// pub view_url: String,
	// pub embed_url: Option<String>,
	pub viewed: bool,
	pub feed_entry_id: String,
	pub feed_id: i32,
	pub produced_date: time_fields::Date,
	pub produced_time: time_fields::OptionTime,
	pub id: i32,
	pub created_at: time_fields::PrimitiveDateTime,
	pub updated_at: time_fields::PrimitiveDateTime,
}

#[cfg(feature="ssr")]
impl EntryOverview {
	fn base_query() -> sea_orm::Select<entry::Entity> {
		entry::Entity::find()
			.select_only()
			.columns(entry::Column::iter().filter(|column| {
				use entry::Column::*;
				!matches!(column, ViewUrl | EmbedUrl )
			}))
	}
	
	pub fn from_query(query: Select<entry::Entity>) -> sea_orm::Selector<SelectModel<Self>> {
		query
			.select_only()
			.columns(entry::Column::iter().filter(|column| {
				use entry::Column::*;
				!matches!(column, ViewUrl | EmbedUrl )
			}))
			.into_model::<Self>()
	}
	
	pub fn query(modifier: impl FnOnce(Select<entry::Entity>) -> Select<entry::Entity>) -> sea_orm::Selector<SelectModel<Self>> {
		Self::query_unordered( |query| {
			let query = query
				.order_by_desc(entry::Column::ProducedDate)
				.order_by_desc(entry::Column::ProducedTime);
			modifier(query)
		} )
	}
	
	pub fn query_unordered(modifier: impl FnOnce(Select<entry::Entity>) -> Select<entry::Entity>) -> sea_orm::Selector<SelectModel<Self>> {
		modifier(Self::base_query())
			.into_model::<Self>()
	}
}

impl Object for EntryOverview {
	fn get_id(&self) -> i32 {
		self.id
	}

	fn get_object_name() -> &'static str where Self: Sized {
		"entry"
	}
}


#[component(transparent)]
pub fn Routes() -> impl IntoView {
	view! {
		<Route path="" view= || view! {
			<Title text="Entries" />
			<Sidebar />
			<main>
				<Outlet/>
			</main>
		}>
			<utils::RouteAlias to="all"/>
			<Route path="all" view=All />
			<Route path="unviewed" view=Unviewed />
		</Route>
	}
}

#[component]
pub fn Sidebar() -> impl IntoView {
	view! {
		<nav class="sidebar">
			<ul>
				<li>
					<A href="all">All</A>
				</li>
				<li>
					<A href="unviewed">Unviewed</A>
				</li>
			</ul>
		</nav>
	}
}



#[server]
pub async fn unviewed() -> Result<Vec<EntryOverview>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	let entries = EntryOverview::query(|q| {
		q.filter(entry::Column::Viewed.eq(false))
	})
		.all(&conn)
		.await?;
	Ok(entries)
}

#[component]
pub fn Unviewed() -> impl IntoView {
	view! {
		<utils::AwaitOk future=unviewed let:entries>
			<Table entries/>
		</utils::AwaitOk>
	}
}


#[server]
pub async fn all_entries() -> Result<Vec<EntryOverview>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	let entries = EntryOverview::query(|q| q)
		.all(&conn)
		.await?;
	Ok(entries)
}

#[component]
pub fn All() -> impl IntoView {
	view! {
		<utils::AwaitOk future=all_entries let:entries>
			<Table entries/>
		</utils::AwaitOk>
	}
}


#[component]
pub fn Table(#[prop(into)] entries: MaybeSignal<Vec<EntryOverview>>) -> impl IntoView {
	view! {
		<table::ObjectTable items = entries />
	}
}