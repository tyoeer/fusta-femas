use leptos::*;
use leptos_router::{Route, A, Outlet};
use leptos_meta::Title;
use entities::prelude::*;
use crate::{table, entry::search::EntryOverview};
use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;


pub mod search;


// ROUTING


#[component(transparent)]
pub fn Routes() -> impl IntoView {
	view! {
		<Route path="/fetch" view=Outlet>
			<Route path="" view= || view! {
				<Title text="Fetch" />
				<main>
					<Outlet/>
				</main>
			}>
				<Route path="" view=|| view! {
					TODO
				} />
			</Route>
			<Route path="/:id" view=SidebarView>
				<utils::RouteAlias to="about" />
				<Route path="about" view = utils::react_id(|id| view! {
						<FieldList id />
				}) />
				<Route path="error" view = utils::react_id(|id| view! {
						<FetchError id />
				}) />
				<Route path="content" view = utils::react_id(|id| view! {
						<FetchedContent id />
				}) />
				<Route path="log" view = utils::react_id(|id| view! {
						<FetchLog id />
				}) />
				<Route path="entries" view = utils::react_id(|id| view! {
						<Entries id />
				}) />
			</Route>
		</Route>
	}
}

#[component]
pub fn SidebarView() -> impl IntoView {
	view! {
		<nav class="sidebar">
			<ul>
				<li>
					<A href="about">About</A>
				</li>
				<li>
					<A href="error">Error</A>
				</li>
				<li>
					<A href="content">Content</A>
				</li>
				<li>
					<A href="log">Log</A>
				</li>
				<li>
					<A href="entries">Entries</A>
				</li>
			</ul>
		</nav>
		<main>
			<Outlet/>
		</main>
	}
}


// INFO


#[server]
pub async fn get_fetch(id: i32) -> Result<fetch::Model, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	fetch::Entity::find_by_id(id)
		.one(&conn)
		.await?
		.ok_or(
			ServerFnError::ServerError("No such fetch".into())
		)
}


#[component]
pub fn FetchError(id: i32) -> impl IntoView {
	view! {
		<utils::AwaitOk future=move || get_fetch(id) let:fetch>
			{
				match fetch.error {
					None => "No error 🤷".into_view(),
					Some(error) => view! {
						<pre>
							{error}
						</pre>
					}.into_view(),
				}
			}
		</utils::AwaitOk>
	}
}
#[component]
pub fn FetchedContent(id: i32) -> impl IntoView {
	view! {
		<utils::AwaitOk future=move || get_fetch(id) let:fetch>
			{
				match fetch.content {
					None => "No content 🤷".into_view(),
					Some(content) => view! {
						<pre>
							{content}
						</pre>
					}.into_view(),
				}
			}
		</utils::AwaitOk>
	}
}
#[component]
pub fn FetchLog(id: i32) -> impl IntoView {
	view! {
		<utils::AwaitOk future=move || get_fetch(id) let:fetch>
			{
				let log = fetch.log;
				if log.is_empty() {
					"Log empty 🤷".into_view()
				} else {
					view! {
						<pre>
							{log}
						</pre>
					}.into_view()
				}
			}
		</utils::AwaitOk>
	}
}

#[server]
async fn get_entries(fetch_id: i32) -> Result<Vec<EntryOverview>,ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	let query = <fetch::Entity as Related<entry::Entity>>::find_related()
		.filter(fetch::Column::Id.eq(fetch_id));
	EntryOverview::from_query(query)
		.all(&conn)
		.await
		.map_err(|e| e.into())	
}

#[component]
pub fn Entries(id: i32) -> impl IntoView {
	view! {
		<utils::AwaitOk future=move || get_entries(id) let:entries>
			<crate::entry::search::Table entries />
		</utils::AwaitOk>
	}
}

#[component]
pub fn FieldList(id: i32) -> impl IntoView {
	view! {
		<utils::AwaitOk future=move || get_fetch(id) let:fetch>
			<table::ObjectFieldValueList object=fetch overloads=vec![
				("error", false, |fetch| view! {
					<table::Reflected value=&fetch.error short=true/>
				}),
				("content", false, |fetch| view! {
					<table::Reflected value=&fetch.content short=true/>
				}),
				("log", false, |fetch| view! {
					<table::Reflected value=&fetch.log short=true/>
				}),
				("feed_id", true, |fetch| {
					//Grab id out because it otherwise will complain about fetch outliving the closure
					//Since the id is i32 which is Copy, it doesn't have that problem
					let id = fetch.feed_id;
					view! {
						<A href=format!("/feed/{id}") class="object_fieldvalue">
							<span class="object_field"> feed_id </span>
							<span class="object_value"> {id} </span>
						</A>
					}.into_view()
				})
			]/>
		</utils::AwaitOk>
	}
}