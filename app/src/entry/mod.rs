use leptos::*;
use leptos_router::{Route, Outlet};

pub mod details;
pub mod search;

// ROUTING


#[component(transparent)]
pub fn Routes() -> impl IntoView {
	view! {
		<Route path="/entry" view=Outlet>
			<search::Routes />
			<details::Routes />
		</Route>
	}
}