use leptos::*;
use leptos_router::A;
use entities::*;
use crate::table::*;
use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;


#[server]
pub async fn all_feeds() -> Result<Vec<feed::Model>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	let feeds = feed::Entity::find().all(&conn).await?;
	Ok(feeds)
}

#[component]
pub fn Search() -> impl IntoView {
	view! {
		<utils::AwaitOk future=all_feeds let:feeds>
			<ObjectTable items = feeds />
		</utils::AwaitOk>
		<A href="new">Create new feed</A>
	}
}