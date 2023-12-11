use leptos::*;
use leptos_router::{Route, Redirect, ActionForm, A, Outlet};
use entities::*;
use crate::table::*;
#[cfg(feature="ssr")]
use sea_orm::*;

mod new;

#[component(transparent)]
pub fn FeedRoutes() -> impl IntoView {
	view! {
		<Route path="/feed" view=Outlet>
			<Route path="" view= || view! {
				<main>
					<Outlet/>
				</main>
			}>
				<Route path="" view=crate::feeds::Feeds />
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
	let button_name = move || {
		if fetch_one.pending().get() {
			"fetching...".to_owned()
		} else {
			match fetch_one.value().get() {
				None => "fetch".to_owned(),
				Some(_) => "fetch again".to_owned(),
			}
		}
	};
	view! {
		<ActionForm action=fetch_one>
			<input type="hidden" name="id" value=id/>
			<input type="submit" value=button_name disabled=move || {fetch_one.pending().get()}/>
		</ActionForm>
		{move || {
			match fetch_one.value().get() {
				Some(Ok(fetch)) => view! {
					<A href=format!("/fetch/{}",fetch.id)>Fetched: {fetch.status.to_string()}</A>
				}, 
				Some(Err(err)) => {
					tracing::error!(fetch_id = ?fetch_one.value().get(), "Error occurred trying to fetch:\n{err}");
					format!("Server error: {err}").into_view()
				},
				None => {
					().into_view()
				},
			}
		}}
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
		<Await future=move || get_feed(id) let:feed>
			{
				feed.clone().map(|feed| view! {
					<ObjectFieldValueList object=&feed />
					<a href=&feed.url target="_blank">{feed.url}</a>
				})
			}
		</Await>
		<FetchFeedButton id />
	}
}

#[server]
pub async fn get_fetches(feed_id: i32) -> Result<Vec<fetch::Model>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	fetch::Entity::find()
		.filter(fetch::Column::FeedId.eq(feed_id))
		.all(&conn)
		.await
		.map_err(|e| e.into())
}

#[component]
pub fn Fetches() -> impl IntoView {
	// Use a closure to opt into the reactive system and respond to changes to id
	|| {
		crate::utils::with_id_param(|feed_id| view! {
			<Await future=move || get_fetches(feed_id) let:fetches>
				{
					fetches.clone().map(|feeds| view! {
						<ObjectTable items = feeds get_id = |feed| feed.id/>
					})
				}
			</Await>
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
			</ul>
		</nav>
		<main>
			<Outlet/>
		</main>
	}
}


// CREATION

// LIST


#[server]
pub async fn get_feeds() -> Result<Vec<feed::Model>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	let feeds = feed::Entity::find().all(&conn).await?;
	Ok(feeds)
}

#[component]
pub fn Feeds() -> impl IntoView {
	view! {
		<Await future=get_feeds let:feeds_res>
			{
				feeds_res.clone().map(|feeds| view! {
					<ObjectTable items = feeds get_id = |feed| feed.id/>
				})
			}
		</Await>
		<A href="new">Create new feed</A>
	}
}