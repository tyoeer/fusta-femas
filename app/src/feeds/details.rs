use leptos::*;
use leptos_router::{Route, ActionForm, A};
use entities::prelude::*;
use crate::table::*;
use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;
#[cfg(feature="ssr")]
use ff_object::View;

use crate::fetch::search::FetchOverview;
use crate::entry::search::EntryOverview;


#[component(transparent)]
pub fn Routes() -> impl IntoView {
	view! {
		<Route path="/:id" view=FeedContext>
			<utils::RouteAlias to="about" />
			<Route path="about" view=FeedInfo/>
			<Route path="fetches" view=Fetches/>
			<Route path="entries" view=Entries/>
			<Route path="tags" view=Tags/>
		</Route>
	}
}

#[component]
pub fn FeedContext() -> impl IntoView {
	view! {
		<utils::ObjectContext getter=get_feed>
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
					<A href="fetches">Fetches</A>
				</li>
				<li>
					<A href="entries">Entries</A>
				</li>
				<li>
					<A href="tags">Tags</A>
				</li>
			</ul>
		</nav>
	}
}


#[server]
pub async fn fetch_one_feed(id: i32) -> Result<fetch::Model, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	let strats = crate::extension!(acquire::strategy_list::StrategyList);
	
	let feed = feed::Entity::find_by_id(id).one(&conn).await?;
	let Some(feed) = feed else {
		return Err(ServerFnError::ServerError(format!("No feed with id {id}")));
	};
	
	let fetch = strats.run(&conn, feed).await;
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
pub fn FetchFeedButton(#[prop(into)] id: MaybeSignal<i32>) -> impl IntoView {
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
pub fn FeedInfo() -> impl IntoView {
	let feed = crate::model!(feed);
	let (id, _) = slice!(feed.id);
	let (url, _) = slice!(feed.url);
	
	use feed::Model as FeedModel;
	
	view! {
		<ObjectFieldValueList<FeedModel> object=feed />
		<a href=move || feed.get().url target="_blank"> {url} </a>
		<FetchFeedButton id />
	}.into()
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
	let feed = crate::model!(feed);
	
	view! {
		<utils::AwaitOk future=move || get_fetches(feed.get().id) let:fetches>
			<ObjectTable items = fetches />
		</utils::AwaitOk>
	}.into()
}


#[server]
pub async fn get_entries(feed_id: i32) -> Result<Vec<EntryOverview>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	EntryOverview::query( |query|
		query.filter(entry::Column::FeedId.eq(feed_id))
	)
		.all(&conn)
		.await
		.map_err(|e| e.into())
}

#[component]
pub fn Entries() -> impl IntoView {
	let feed = crate::model!(feed);
	
	view! {
		<utils::AwaitOk future=move || get_entries(feed.get().id) let:entries>
			<crate::entry::search::Table entries />
		</utils::AwaitOk>
	}.into()
}


#[server]
pub async fn get_tags(feed_id: i32) -> Result<Vec<tag::Model>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	<feed::Entity as Related<tag::Entity>>::find_related()
		.filter(feed::Column::Id.eq(feed_id))
		.all(&conn)
		.await
		.map_err(|e| e.into())
}

#[server]
pub async fn get_available_tags(feed_id: i32) -> Result<Vec<tag::Model>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	
	tag::Entity::find()
		.filter(
			tag::Column::Id.not_in_subquery(
				<feed::Entity as Related<tag::Entity>>::find_related()
					.filter(feed::Column::Id.eq(feed_id))
					.select_only()
					.select_column(tag::Column::Id)
					.into_query()
			)
		)
		.all(&conn)
		.await
		.map_err(|e| e.into())
}

#[server]
async fn add_tag(feed_id: i32, tag_id: i32) -> Result<(), ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	
	let mut feed_tag = feed_tag::ActiveModel::new();
	feed_tag.feed_id = Set(feed_id);
	//TODO validate tag id
	feed_tag.tag_id = Set(tag_id);
	
	feed_tag.insert(&conn).await?;
	
	Ok(())
}

#[component]
pub fn Tags() -> impl IntoView {
	let feed = crate::model!(feed);
	let add_tag = create_server_action::<AddTag>();
	let feed_id = move || feed.get().id;
	
	let resource_input = move || (feed_id(), add_tag.version().get());
	
	let feed_tags = Resource::new(
		resource_input,
		|(feed_id, _)| get_tags(feed_id)
	);
	
	let available_tags = Resource::new(
		resource_input,
		|(feed_id, _)| get_available_tags(feed_id)
	);
	
	view! {
		<ActionForm action=add_tag>
			<input type="hidden" name="feed_id" value=feed_id/>
			<select name="tag_id">
				<Suspense
					fallback = || view!{ <option selected=true disabled=true> "Loading..." </option> }
				>
					<ErrorBoundary fallback = |errors| view!{ <crate::app::ErrorsView errors /> } >
						{
							move || available_tags.get().map(
								|tags_res| tags_res.map(
									|tags| view! {
										<For
											each=move || tags.clone()
											key=|tag| tag.id
											let:tag
										>
											<option value=tag.id> {tag.title} </option>
										</For>
									}
								)
							)
						}
					</ErrorBoundary>
				</Suspense>
				// <utils::AwaitOk future=move || get_available_tags(feed_id()) let:tags>
				// 	<For
				// 		each=move || tags.clone()
				// 		key=|tag| tag.id
				// 		let:tag
				// 	>
				// 		<option value=tag.id> {tag.title} </option>
				// 	</For>
				// </utils::AwaitOk>
			</select>
			
			<utils::FormSubmit action=add_tag button="add tag"/>
		</ActionForm>
		
		<Transition
			fallback = || view! {<div>"Loading..."</div>}
		>
			<ErrorBoundary fallback = |errors| view!{ <crate::app::ErrorsView errors /> } >
				{
					move || feed_tags.get().map(
						|tags_res| tags_res.map(
							|tags| view! {
								<crate::tag::search::Table tags />
							}
						)
					)
				}
			</ErrorBoundary>
		</Transition>
	}.into()
}