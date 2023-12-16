use leptos::*;
// use leptos_router::A;
use entities::*;
use crate::table;
use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;

#[server]
pub async fn all_entries() -> Result<Vec<entry::Model>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	let feeds = entry::Entity::find().all(&conn).await?;
	Ok(feeds)
}

#[component]
pub fn Search() -> impl IntoView {
	view! {
		<utils::AwaitOk future=all_entries let:entries>
			<Table entries/>
		</utils::AwaitOk>
	}
}

#[component]
pub fn Table(#[prop(into)] entries: MaybeSignal<Vec<entry::Model>>) -> impl IntoView {
	view! {
		<table::ObjectTable items = entries />
	}
}