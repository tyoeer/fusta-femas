use leptos::*;
use leptos_router::{Route, A};
use entities::prelude::*;
use crate::table::*;
use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;

// use crate::entry::search::EntryOverview;


#[component(transparent)]
pub fn Routes() -> impl IntoView {
	view! {
		<Route path="/:id" view=FeedContext>
			<utils::RouteAlias to="about" />
			<Route path="about" view=TagInfo/>
			// <Route path="entries" view=Entries/>
		</Route>
	}
}

#[component]
pub fn FeedContext() -> impl IntoView {
	view! {
		<utils::ObjectContext getter=get_tag>
			<Sidebar />
		</utils::ObjectContext>
	}
}

#[component]
pub fn Sidebar() -> impl IntoView {
	view! {
		<nav class="sidebar">
			<ul>
				<li>
					<A href="about">About</A>
				</li>
				// <li>
				// 	<A href="entries">Entries</A>
				// </li>
			</ul>
		</nav>
	}
}


#[server]
pub async fn get_tag(id: i32) -> Result<tag::Model, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	tag::Entity::find_by_id(id)
		.one(&conn)
		.await?
		.ok_or(
			ServerFnError::ServerError("No such tag".into())
		)
}

#[component]
pub fn TagInfo() -> impl IntoView {
	let tag = crate::model!(tag);
	
	use tag::Model as TagModel;
	
	view! {
		<ObjectFieldValueList<TagModel> object=tag />
	}.into()
}


// #[server]
// pub async fn get_entries(feed_id: i32) -> Result<Vec<EntryOverview>, ServerFnError> {
// 	let conn = crate::extension!(DatabaseConnection);
// 	EntryOverview::query( |query|
// 		query.filter(entry::Column::FeedId.eq(feed_id))
// 	)
// 		.all(&conn)
// 		.await
// 		.map_err(|e| e.into())
// }

// #[component]
// pub fn Entries() -> impl IntoView {
// 	let feed = crate::model!(feed);
	
// 	view! {
// 		<utils::AwaitOk future=move || get_entries(feed.get().id) let:entries>
// 			<crate::entry::search::Table entries />
// 		</utils::AwaitOk>
// 	}.into()
// }