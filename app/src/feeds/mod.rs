use leptos::*;
use leptos_router::{Route, Redirect, ActionForm, A, Outlet};
use entities::*;
use crate::table::*;
#[cfg(feature="ssr")]
use sea_orm::*;


#[component(transparent)]
pub fn FeedRoutes() -> impl IntoView {
	view! {
		<Route path="/feeds" view=Outlet>
			<Route path="" view= || view! {
				<main>
					<Outlet/>
				</main>
			}>
				<Route path="" view=crate::feeds::Feeds />
				<Route path="/new" view=crate::feeds::FeedCreator />
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
pub async fn fetch_one_feed(id: i32) -> Result<i32, ServerFnError> {
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
	
	Ok(fetch.id)
}

#[component]
pub fn FetchFeedButton(id: i32) -> impl IntoView {
	let fetch_one = create_server_action::<FetchOneFeed>();
	let button_name = move || {
		if fetch_one.pending().get() {
			"fetching...".to_owned()
		} else {
			match fetch_one.value().get() {
				None => {
					"fetch".to_owned()
				},
				Some(res) => match res {
					Ok(id) => format!("fetched: {id}"),
					Err(err) => format!("server error: {err}"),
				}
			}
		}
	};
	view! {
		<ActionForm action=fetch_one>
			<input type="hidden" name="id" value=id/>
			<input type="submit" value=button_name/>
		</ActionForm>
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
	let id = crate::utils::with_id_param(|id| id).expect("No id");
	view! {
		<Await future=move || get_fetches(id) let:fetches>
			{
				fetches.clone().map(|feeds| view! {
					<ObjectTable items = feeds get_id = |feed| feed.id/>
				})
			}
		</Await>
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


#[server]
pub async fn new_feed(name: String, url: String, strategy: String) -> Result<i32, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	let mut new = feed::ActiveModel::new();
	new.name = Set(name);
	new.url = Set(url);
	//TODO validate
	new.strategy = Set(strategy);
	let inserted = new.insert(&conn).await?;
	Ok(inserted.id)
}

#[component]
pub fn FeedCreator() -> impl IntoView {
	let new_feed = create_server_action::<NewFeed>();
	let button_name = move || {
		if new_feed.pending().get() {
			"creating...".to_owned()
		} else {
			match new_feed.value().get() {
				None => {
					"Create".to_owned()
				},
				Some(res) => match res {
					Ok(id) => format!("created : {id}"),
					Err(err) => format!("server error: {err}"),
				}
			}
		}
	};
	view! {
		<ActionForm action=new_feed>
			<input type="text" name="name" />
			<input type="text" name="url" />
			<select name="strategy">
				<Await future=super::strategies::get_strategies let:strats>
					{
						strats.clone().map(|strats| {
							view! {
								<For
									each=move || strats.clone()
									key=|s| s.clone()
									let:strat
								>
									<option value=strat.clone()> {strat} </option>
								</For>
							}
						})
					}
					
				</Await>
			</select>
			<input type="submit" value=button_name/>
		</ActionForm>
	}
}


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