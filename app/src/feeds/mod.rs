use leptos::*;
use leptos_router::{Route, Redirect, ActionForm, A, Outlet};
use entities::prelude::*;
use crate::table::*;
use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;

use crate::fetch::search::FetchOverview;

pub mod new;
pub mod search;

#[component(transparent)]
pub fn FeedRoutes() -> impl IntoView {
	view! {
		<Route path="/feed" view=Outlet>
			<Route path="" view= || view! {
				<main>
					<Outlet/>
				</main>
			}>
				<Route path="" view=search::Search />
				<Route path="/new" view=new::FeedCreator />
			</Route>
			<Route path="/:id" view=crate::feeds::FeedOverview>
				<Route path="" view=|| view! { <Redirect path="about"/> }/>
				<Route path="about" view = || {
					crate::utils::with_id_param(|id| view! {
						<crate::feeds::FeedInfo id />
					})
				} />
				<Route path="fetches" view=Fetches/>
				<Route path="entries" view=Entries/>
			</Route>
		</Route>
	}
}


// FEED INFO


#[server]
pub async fn fetch_one_feed(id: i32) -> Result<fetch::Model, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	let strats = crate::extension!(acquire::strategy_list::StrategyList);
	
	let feed = feed::Entity::find_by_id(id).one(&conn).await?;
	let Some(feed) = feed else {
		return Err(ServerFnError::ServerError(format!("No feed with id {id}")));
	};
	
	let fetch = strats.run(conn, feed).await;
	let fetch = match fetch {
		Ok(f) => f,
		Err(e) => {
			tracing::error!("{e:?}");
			return Err(e.into());
		}
	};
	
	Ok(fetch)
}

#[component]
pub fn FetchFeedButton(id: i32) -> impl IntoView {
	let fetch_one = create_server_action::<FetchOneFeed>();
	view! {
		<ActionForm action=fetch_one>
			<input type="hidden" name="id" value=id/>
			<utils::FormSubmit button="fetch" action=fetch_one/>
		</ActionForm>
		<utils::FormResult action=fetch_one let:fetch>
			<A href=format!("/fetch/{}", fetch.id)>"Fetched: " {fetch.status.to_string()}</A>
		</utils::FormResult>
	}
}

#[server]
pub async fn get_feed(id: i32) -> Result<feed::Model, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	feed::Entity::find_by_id(id)
		.one(&conn)
		.await?
		.ok_or(
			ServerFnError::ServerError("No such feed".into())
		)
}

#[component]
pub fn FeedInfo(id: i32) -> impl IntoView {
	view! {
		<utils::AwaitOk future=move || get_feed(id) let:feed>
			<ObjectFieldValueList object=&feed />
			<a href=&feed.url target="_blank">{feed.url}</a>
		</utils::AwaitOk>
		<FetchFeedButton id />
	}
}

#[server]
pub async fn get_fetches(feed_id: i32) -> Result<Vec<FetchOverview>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	FetchOverview::query(|q| q.filter(fetch::Column::FeedId.eq(feed_id)))
		.all(&conn)
		.await
		.map_err(|e| e.into())
}

#[component]
pub fn Fetches() -> impl IntoView {
	// Use a closure to opt into the reactive system and respond to changes to id
	|| {
		crate::utils::with_id_param(|feed_id| view! {
			<utils::AwaitOk future=move || get_fetches(feed_id) let:fetches>
				<ObjectTable items = fetches />
			</utils::AwaitOk>
		})
	}
}

#[server]
pub async fn get_entries(feed_id: i32) -> Result<Vec<entry::Model>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	entry::Entity::find()
		.filter(entry::Column::FeedId.eq(feed_id))
		.all(&conn)
		.await
		.map_err(|e| e.into())
}

#[component]
pub fn Entries() -> impl IntoView {
	// Use a closure to opt into the reactive system and respond to changes to id
	|| {
		crate::utils::with_id_param(|feed_id| view! {
			<utils::AwaitOk future=move || get_entries(feed_id) let:entries>
				<crate::entry::search::Table entries />
			</utils::AwaitOk>
		})
	}
}

#[component]
pub fn FeedOverview() -> impl IntoView {
	view! {
		<nav class="sidebar">
			<ul>
				<li>
					<A href="about">About</A>
				</li>
				<li>
					<A href="fetches">Fetches</A>
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