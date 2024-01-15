use leptos::*;
use leptos_router::A;
use entities::prelude::*;
use crate::table::*;
use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;


#[server]
pub async fn all_tags() -> Result<Vec<tag::Model>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	let feeds = tag::Entity::find().all(&conn).await?;
	Ok(feeds)
}

#[component]
pub fn Search() -> impl IntoView {
	view! {
		<utils::AwaitOk future=all_tags let:tags>
			<Table tags />
		</utils::AwaitOk>
		<A href="new">Create new tag</A>
	}
}

#[component]
pub fn Table(#[prop(into)] tags: MaybeSignal<Vec<tag::Model>>) -> impl IntoView {
	view! {
		<ObjectTable items = tags />
	}
}