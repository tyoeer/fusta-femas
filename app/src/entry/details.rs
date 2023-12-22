use leptos::*;
use leptos_router::{A, Outlet, Route, Redirect};
use entities::prelude::*;
use crate::table;
use crate::fetch::search::FetchOverview;
use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;

#[component(transparent)]
pub fn Routes() -> impl IntoView {
	view! {
		<Route path="/:id" view=ObjectContext>
			<Route path="" view=|| view! { <Redirect path="about"/> }/>
			<Route path="about" view = About />
			<Route path="embedded" view = Embed />
			<Route path="fetches" view = Fetches />
		</Route>
	}
}

#[component]
pub fn ObjectContext() -> impl IntoView {
	|| crate::utils::with_id_param(|id| view! {
		<Navbar />
		<main>
			<utils::AwaitOk future=move || get_entry(id) let:entry>
				{
					provide_context(entry);
					
					view! {
						<Outlet/>
					}
				}
			</utils::AwaitOk>
		</main>
	})
}

#[component]
pub fn Navbar() -> impl IntoView {
	view! {
		<nav class="sidebar">
			<ul>
				<li>
					<A href="about">About</A>
				</li>
				<li>
					<A href="embedded">Embedded</A>
				</li>
				<li>
					<A href="fetches">Fetches</A>
				</li>
			</ul>
		</nav>
	}
}


#[server]
pub async fn get_entry(id: i32) -> Result<entry::Model, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	entry::Entity::find_by_id(id)
		.one(&conn)
		.await?
		.ok_or(
			ServerFnError::ServerError("No such feed".into())
		)
}

#[component]
pub fn About() -> impl IntoView {
	let entry = crate::model!(entry);
	
	view! {
		<table::ObjectFieldValueList object=&entry />
	}.into()
}

#[component]
pub fn Embed() -> impl IntoView {
	let entry = crate::model!(entry);
	
	let maybe_url = entry.embed_url;
	maybe_url.map(|mut url| {
		if !url.contains("://") {
			url = format!("https://{url}");
		}
		view! {
			<iframe class="grow" src=url />
		}.into_view()
	}).unwrap_or_else(|| {
		view! {
			"Entry has no embed url specified 🤷"
		}.into_view()
	}).into()
}

#[server]
pub async fn get_fetches(entry_id: i32) -> Result<Vec<FetchOverview>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	let query = <entry::Entity as Related<fetch::Entity>>::find_related()
		.filter(entry::Column::Id.eq(entry_id));
	FetchOverview::from_query(query)
		.all(&conn)
		.await
		.map_err(|e| e.into())	
}

#[component]
pub fn Fetches() -> impl IntoView {
	let entry = crate::model!(entry);
	
	view! {
		<utils::AwaitOk future=move || get_fetches(entry.id) let:fetches>
			<table::ObjectTable items = fetches />
		</utils::AwaitOk>
	}.into()
}