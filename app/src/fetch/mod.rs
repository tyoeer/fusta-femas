use leptos::*;
use leptos_router::{Route, Redirect, A, Outlet};
use entities::*;
use crate::table;
use crate::utils;
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
		<utils::AwaitOk future=move || get_fetch(id) let:fetch>
			{
				match fetch.error {
					None => "No error 🤷".into_view(),
					Some(error) => view! {
						<pre>
							{error}
						</pre>
					}.into_view(),
				}
			}
		</utils::AwaitOk>
	}
}
#[component]
pub fn FetchedContent(id: i32) -> impl IntoView {
	view! {
		<utils::AwaitOk future=move || get_fetch(id) let:fetch>
			{
				match fetch.content {
					None => "No content 🤷".into_view(),
					Some(content) => view! {
						<pre>
							{content}
						</pre>
					}.into_view(),
				}
			}
		</utils::AwaitOk>
	}
}

#[component]
pub fn FieldList(id: i32) -> impl IntoView {
	view! {
		<utils::AwaitOk future=move || get_fetch(id) let:fetch>
			<table::ObjectFieldValueList object=&fetch overloads=vec![
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
		</utils::AwaitOk>
	}
}