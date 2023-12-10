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
				<Route path="error" view = || {
					crate::utils::with_id_param(|id| view! {
						<FetchError id />
					})
				} />
				<Route path="content" view = || {
					crate::utils::with_id_param(|id| view! {
						<FetchedContent id />
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
				<li>
					<A href="error">Error</A>
				</li>
				<li>
					<A href="content">Content</A>
				</li>
			</ul>
		</nav>
		<main>
			<Outlet/>
		</main>
	}
}


// INFO


#[server]
pub async fn get_fetch(id: i32) -> Result<fetch::Model, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	fetch::Entity::find_by_id(id)
		.one(&conn)
		.await?
		.ok_or(
			ServerFnError::ServerError("No such fetch".into())
		)
}


#[component]
pub fn FetchError(id: i32) -> impl IntoView {
	view! {
		<Await future=move || get_fetch(id) let:fetch_res>
			<pre>
				{
					fetch_res.clone().map(|fetch| {
						fetch.error.unwrap_or("No error 🤷".to_owned())
					})
				}
			</pre>
		</Await>
	}
}
#[component]
pub fn FetchedContent(id: i32) -> impl IntoView {
	view! {
		<Await future=move || get_fetch(id) let:fetch_res>
			<pre>
				{
					fetch_res.clone().map(|fetch| {
						fetch.content.unwrap_or("No content 🤷".to_owned())
					})
				}
			</pre>
		</Await>
	}
}

#[component]
pub fn FieldList(id: i32) -> impl IntoView {
	view! {
		<Await future=move || get_fetch(id) let:fetch>
			{
				fetch.clone().map(|feed| view! {
					<table::ObjectFieldValueList object=&feed overloads=vec![
						("error", false, |fetch| view! {
							<table::Reflected value=&fetch.error short=true/>
						}),
						("content", false, |fetch| view! {
							<table::Reflected value=&fetch.content short=true/>
						}),
						("feed_id", true, |fetch| {
							//Grab id out because it otherwise will complain about fetch outliving the closure
							//Since the id is i32 which is Copy, it doesn't have that problem
							let id = fetch.feed_id;
							view! {
								<A href=format!("/feed/{id}") class="object_fieldvalue">
									<span class="object_field"> feed_id </span>
									<span class="object_value"> {id} </span>
								</A>
							}.into_view()
						})
					]/>
				})
			}
		</Await>
	}
}