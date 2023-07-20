use leptos::*;
use leptos_router::ActionForm;
#[cfg(feature="ssr")]
use sea_entities::*;
#[cfg(feature="ssr")]
use sea_orm::*;
use serde::*;

#[derive(Debug,Clone, PartialEq,Eq, Serialize, Deserialize)]
pub struct FeedInfo {
	id: i32,
	name: String,
	url: String,
}

#[cfg(feature="ssr")]
impl From<feed::Model> for FeedInfo {
	fn from(model: feed::Model) -> Self {
		let mut url = model.url;
		if !url.contains("://") {
			url = format!("https://{}",url);
		}
		Self {
			id: model.id,
			name: model.name,
			url,
		}
	}
}

#[server(GetFeeds, "/api")]
pub async fn get_feeds(cx: Scope) -> Result<Vec<FeedInfo>, ServerFnError> {
	let conn = use_context::<DatabaseConnection>(cx)
		.ok_or_else(|| ServerFnError::ServerError("Missing DB connection pool".into()))?;
	let feeds = feed::Entity::find().all(&conn).await?;
	let urls = feeds.into_iter().map(|f| f.into()).collect();
	Ok(urls)
}

#[server(FetchOne, "/api")]
pub async fn fetch_one_feed(cx: Scope, id: i32) -> Result<i32, ServerFnError> {
	let conn = use_context::<DatabaseConnection>(cx)
		.ok_or_else(|| ServerFnError::ServerError("Missing DB connection pool".into()))?;
	let strats = super::get_strats(cx).await?;
	
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
pub fn Feed(cx: Scope, fi: FeedInfo) -> impl IntoView {
	let fetch_one = create_server_action::<FetchOne>(cx);
	let button_name = move || {
		match fetch_one.value().get() {
			None => {
				if fetch_one.pending().get() {
					"fecthing...".to_owned()
				} else {
					"fetch".to_owned()
				}
			},
			Some(res) => match res {
				Ok(id) => format!("fetched: {id}"),
				Err(err) => format!("server error: {err}"),
			}
		}
	};
	view! {cx,
		<span class="table_cell">{fi.name}</span>
		<span class="table_cell"><a href=&fi.url target="_blank">{fi.url}</a></span>
		<span class="table_cell">
			<ActionForm action=fetch_one>
				<input type="hidden" name="id" value=fi.id/>
				<input type="submit" value=button_name/>
			</ActionForm>
		</span>
	}
}

#[component]
pub fn Feeds(cx: Scope) -> impl IntoView {
	view! {cx,
		<Await future=get_feeds bind:feeds>
			<ul class="table">
				{
					feeds.clone().map(|vec| {
						vec.into_iter()
							.map(|e| view! {cx, <li class="table_row"><Feed fi=e/></li>})
							.collect::<Vec<_>>()
					})
				}
			</ul>
		</Await>
	}
}