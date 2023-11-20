use leptos::*;
use leptos_router::{ActionForm, A};
use entities::*;
use crate::table::*;
#[cfg(feature="ssr")]
use leptos_axum::extractor;
#[cfg(feature="ssr")]
use axum::Extension;
#[cfg(feature="ssr")]
use sea_orm::*;



#[server]
pub async fn get_feeds() -> Result<Vec<feed::Model>, ServerFnError> {
	let conn = extractor::<Extension<DatabaseConnection>>().await.map(|ext| ext.0)?;
	let feeds = feed::Entity::find().all(&conn).await?;
	Ok(feeds)
}

#[server]
pub async fn fetch_one_feed(id: i32) -> Result<i32, ServerFnError> {
	let conn = extractor::<Extension<DatabaseConnection>>().await.map(|ext| ext.0)?;
	let strats = super::strategies::get_strats().await?;
	
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
pub fn Feed(feed: feed::Model) -> impl IntoView {
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
		<TableRow item=&feed/>
		<span class="table_cell">
			<a href=&feed.url target="_blank">{feed.url}</a>
		</span>
		<span class="table_cell">
			<ActionForm action=fetch_one>
				<input type="hidden" name="id" value=feed.id/>
				<input type="submit" value=button_name/>
			</ActionForm>
		</span>
	}
}

#[server]
pub async fn new_feed(name: String, url: String, strategy: String) -> Result<i32, ServerFnError> {
	let conn = extractor::<Extension<DatabaseConnection>>().await.map(|ext| ext.0)?;
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

#[component]
pub fn Feeds() -> impl IntoView {
	view! {
		<Await future=get_feeds let:feeds>
			<ul class="table">
				<TableHeader struct_info={struct_info::<feed::Model>()} />
				{
					feeds.clone().map(|vec| {
						vec.into_iter()
							.map(|feed| view! {<li class="table_row"><Feed feed=feed/></li>})
							.collect::<Vec<_>>()
					})
				}
			</ul>
		</Await>
		<A href="new">Create new feed</A>
	}
}