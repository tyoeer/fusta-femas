use leptos::*;
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
		if let None = url.find("://") {
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

#[component]
pub fn Feed(cx: Scope, fi: FeedInfo) -> impl IntoView {
	view! {cx,
		<span class="table_cell">{fi.name}</span>
		<span class="table_cell"><a href=&fi.url target="_blank">{fi.url}</a></span>
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