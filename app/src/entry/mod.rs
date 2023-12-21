use leptos::*;
use leptos_router::{Route, Outlet};

pub mod details;
pub mod search;

// ROUTING


#[component(transparent)]
pub fn Routes() -> impl IntoView {
	view! {
		<Route path="/entry" view=Outlet>
			<Route path="" view= || view! {
				<main>
					<Outlet/>
				</main>
			}>
				<Route path="" view=search::Search />
			</Route>
			<details::Routes />
		</Route>
	}
}