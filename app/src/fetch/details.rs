use leptos::*;
use leptos_router::{Route, A};
use entities::prelude::*;
use crate::{table, entry::search::EntryOverview};
use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;


#[component(transparent)]
pub fn Routes() -> impl IntoView {
	view! {
		<Route path="/:id" view=FetchContext>
			<utils::RouteAlias to="about" />
			<Route path="about" view = FieldList />
			<Route path="error" view = FetchError />
			<Route path="content" view = FetchedContent />
			<Route path="log" view = FetchLog />
			<Route path="entries" view = Entries />
		</Route>
	}
}


#[component]
pub fn FetchContext() -> impl IntoView {
	view! {
		<utils::ObjectContext getter=get_fetch>
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
pub fn FetchError() -> impl IntoView {
	let fetch = crate::model!(fetch);
	
	let react = move || {
		match fetch.get().error {
			None => "No error 🤷".into_view(),
			Some(error) => view! {
				<pre>
					{error}
				</pre>
			}.into_view(),
		}
	};
	
	Some(react)
}
#[component]
pub fn FetchedContent() -> impl IntoView {
	let fetch = crate::model!(fetch);
	
	let react = move || {
		match fetch.get().content {
			None => "No content 🤷".into_view(),
			Some(content) => view! {
				<pre>
					{content}
				</pre>
			}.into_view(),
		}
	};
	
	Some(react)
}
#[component]
pub fn FetchLog() -> impl IntoView {
	let fetch = crate::model!(fetch);
	
	let react = move || {
		let log = fetch.get().log;
		if log.is_empty() {
			"Log empty 🤷".into_view()
		} else {
			view! {
				<pre>
					{log}
				</pre>
			}.into_view()
		}
	};
	
	Some(react)
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
pub fn Entries() -> impl IntoView {
	let fetch = crate::model!(fetch);
	
	view! {
		<utils::AwaitOk future=move || get_entries(fetch.get().id) let:entries>
			<crate::entry::search::Table entries />
		</utils::AwaitOk>
	}.into()
}

#[component]
pub fn FieldList() -> impl IntoView {
	let fetch = crate::model!(fetch);
	
	use fetch::Model as FetchModel;
	
	view! {
		<table::ObjectFieldValueList<FetchModel> object=fetch overloads=vec![
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
	}.into()
}