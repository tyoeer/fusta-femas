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
type Entity = entry::Entity;

#[cfg(feature="ssr")]
impl EntryOverview {
	fn columns() -> impl Iterator<Item = impl sea_orm::ColumnTrait> {
		entry::Column::iter().filter(|column| {
			use entry::Column::*;
			!matches!(column, ViewUrl | EmbedUrl )
		})
	}
	
	fn order(query: Select<Entity>) -> Select<Entity> {
		query
			.order_by_desc(entry::Column::ProducedDate)
			.order_by_desc(entry::Column::ProducedTime)
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

impl ff_object::Object for EntryOverview {
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