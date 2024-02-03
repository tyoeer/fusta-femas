use leptos::*;
use leptos_router::{Route, ActionForm, A};
use entities::prelude::*;
use crate::table::*;
use crate::utils;
use ff_object::{Object, ref_signal};
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
pub async fn fetch_one_feed(feed: feed::Ref) -> Result<fetch::Model, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	let strats = crate::extension!(acquire::strategy_list::StrategyList);
	
	let maybe_feed = feed.find().one(&conn).await?;
	let Some(feed) = maybe_feed else {
		return Err(ServerFnError::ServerError(format!("No feed with id {}", feed.id())));
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
pub fn FetchFeedButton(#[prop(into)] feed: MaybeSignal<feed::Ref>) -> impl IntoView {
	let fetch_one = create_server_action::<FetchOneFeed>();
	view! {
		<ActionForm action=fetch_one>
			<input type="hidden" name="feed" value=move || feed.get().id()/>
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
	let feed_ref = ref_signal(feed);
	let (url, _) = slice!(feed.url);
	
	use feed::Model as FeedModel;
	
	view! {
		<ObjectFieldValueList<FeedModel> object=feed />
		<a href=move || feed.get().url target="_blank"> {url} </a>
		<FetchFeedButton feed=feed_ref />
	}.into()
}


#[server]
pub async fn get_fetches(feed: feed::Ref) -> Result<Vec<FetchOverview>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	FetchOverview::query(|q| feed.filter_related(q))
		.all(&conn)
		.await
		.map_err(|e| e.into())
}

#[component]
pub fn Fetches() -> impl IntoView {
	let feed_ref = ref_signal(crate::model!(feed));
	
	view! {
		<utils::AwaitOk future=move || get_fetches(feed_ref.get()) let:fetches>
			<ObjectTable items = fetches />
		</utils::AwaitOk>
	}.into()
}


#[server]
pub async fn get_entries(feed: feed::Ref) -> Result<Vec<EntryOverview>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	EntryOverview::query( |query|
		feed.filter_related(query)
	)
		.all(&conn)
		.await
		.map_err(|e| e.into())
}

#[component]
pub fn Entries() -> impl IntoView {
	let feed = crate::model!(feed);
	
	view! {
		<utils::AwaitOk future=move || get_entries(feed.get().get_ref()) let:entries>
			<crate::entry::search::Table entries />
		</utils::AwaitOk>
	}.into()
}


#[server]
pub async fn get_tags(feed: feed::Ref) -> Result<Vec<tag::Model>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	feed.find_related::<tag::Entity>()
		.all(&conn)
		.await
		.map_err(|e| e.into())
}

#[server]
pub async fn get_available_tags(feed: feed::Ref) -> Result<Vec<tag::Model>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	
	tag::Entity::find()
		.filter(
			tag::Column::Id.not_in_subquery(
				feed.find_related::<tag::Entity>()
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
	let feed_ref = ref_signal(feed);
	
	let resource_input = move || (feed_ref.get(), add_tag.version().get());
	
	let feed_tags = Resource::new(
		resource_input,
		|(feed_ref, _)| get_tags(feed_ref)
	);
	
	let available_tags = Resource::new(
		resource_input,
		|(feed_ref, _)| get_available_tags(feed_ref)
	);
	
	view! {
		<utils::ResourceOk
			fallback = || view!{ <option selected=true disabled=true> "Loading..." </option> }
			resource = available_tags
			suspense = true
			let:tags
		>
			{
				//TODO surely there's a better way to do this
				let tags_stored = store_value(tags);
				view! {
					<Show when = move || !tags_stored.with_value(|tags| tags.is_empty())>
						<ActionForm action=add_tag>
							<input type="hidden" name="feed_id" value=move || feed_ref.get().id()/>
							<select name="tag_id">
								<For
									each=move || tags_stored.get_value()
									key=|tag| tag.id
									let:tag
								>
									<option value=tag.id> {tag.title} </option>
								</For>
							</select>
							<utils::FormSubmit action=add_tag button="add tag"/>
						</ActionForm>
					</Show>
					<Show when=move || tags_stored.with_value(|tags| tags.is_empty())>
						<p> "No tags left to add" </p>
					</Show>
				}
			}
		</utils::ResourceOk>
		
		<utils::ResourceOk
			fallback = || view! {<div>"Loading..."</div>}
			resource = feed_tags
			let:tags
		>
			<crate::tag::search::Table tags />
		</utils::ResourceOk>
	}.into()
}