use leptos::*;
#[cfg(feature="ssr")]
use sea_entities::*;
#[cfg(feature="ssr")]
use sea_orm::*;


#[server(GetFeeds, "/api")]
pub async fn get_feeds(cx: Scope) -> Result<Vec<String>, ServerFnError> {
	let conn = use_context::<DatabaseConnection>(cx)
		.ok_or_else(|| ServerFnError::ServerError("Missing DB connection pool".into()))?;
	let feeds = feed::Entity::find().all(&conn).await?;
	let urls = feeds.into_iter().map(|f| f.url).collect();
	Ok(urls)
}

#[component]
pub fn Feeds(cx: Scope) -> impl IntoView {
	view! {cx,
		<Await future=get_feeds bind:feeds>
			<ul>
				{
					feeds.clone().map(|vec| {
						vec.into_iter()
							.map(|e| view! {cx, <li>{e}</li>})
							.collect::<Vec<_>>()
					})
				}
			</ul>
		</Await>
	}
}