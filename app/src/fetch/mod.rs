use leptos::*;
use leptos_router::{Route, Redirect, A, Outlet};
use entities::*;
use crate::table;
#[cfg(feature="ssr")]
use sea_orm::*;


// ROUTING


#[component(transparent)]
pub fn Routes() -> impl IntoView {
	view! {
		<Route path="/fetch" view=Outlet>
			<Route path="" view= || view! {
				<main>
					<Outlet/>
				</main>
			}>
				<Route path="" view=|| view! {
					TODO
				} />
			</Route>
			<Route path="/:id" view=SidebarView>
				<Route path="" view=|| view! { <Redirect path="about"/> }/>
				<Route path="about" view = || {
					crate::utils::with_id_param(|id| view! {
						<FieldList id />
					})
				} />
			</Route>
		</Route>
	}
}

#[component]
pub fn SidebarView() -> impl IntoView {
	view! {
		<nav class="sidebar">
			<ul>
				<li>
					<A href="about">About</A>
				</li>
			</ul>
		</nav>
		<main>
			<Outlet/>
		</main>
	}
}


// ABOUT


#[server]
pub async fn get_fetch(id: i32) -> Result<fetch::Model, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	fetch::Entity::find_by_id(id)
		.one(&conn)
		.await?
		.ok_or(
			ServerFnError::ServerError("No such feed".into())
		)
}

#[component]
pub fn FieldList(id: i32) -> impl IntoView {
	view! {
		<Await future=move || get_fetch(id) let:fetch>
			{
				fetch.clone().map(|feed| view! {
					<table::ObjectFieldValueList object=&feed />
				})
			}
		</Await>
	}
}