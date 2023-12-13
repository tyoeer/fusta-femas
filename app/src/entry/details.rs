use leptos::*;
use leptos_router::{A, Outlet};
use entities::*;
use crate::table;
use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;


#[component]
pub fn Navbar() -> impl IntoView {
	view! {
		<nav class="sidebar">
			<ul>
				<li>
					<A href="about">About</A>
				</li>
				<li>
					<A href="embedded">Embedded</A>
				</li>
			</ul>
		</nav>
		<main>
			<Outlet/>
		</main>
	}
}


#[server]
pub async fn get_entry(id: i32) -> Result<entry::Model, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	entry::Entity::find_by_id(id)
		.one(&conn)
		.await?
		.ok_or(
			ServerFnError::ServerError("No such feed".into())
		)
}

#[component]
pub fn About(id: i32) -> impl IntoView {
	view! {
		<utils::AwaitOk future=move || get_entry(id) let:entry>
			<table::ObjectFieldValueList object=&entry />
		</utils::AwaitOk>
	}
}

#[component]
pub fn Embed(id: i32) -> impl IntoView {
	view! {
		<utils::AwaitOk future=move || get_entry(id) let:entry>
			{
				let maybe_url = entry.embed_url;
				maybe_url.map(|mut url| {
					if !url.contains("://") {
						url = format!("https://{url}");
					}
					view! {
						<iframe class="grow" src=url />
					}.into_view()
				}).unwrap_or_else(|| {
					view! {
						"Entry has no embed url specified 🤷"
					}.into_view()
				})
			}
		</utils::AwaitOk>
	}
}