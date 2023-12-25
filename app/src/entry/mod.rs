use leptos::*;
use leptos_router::{Route, Outlet};
use leptos_meta::Title;

pub mod details;
pub mod search;

// ROUTING


#[component(transparent)]
pub fn Routes() -> impl IntoView {
	view! {
		<Route path="/entry" view=Outlet>
			<Route path="" view= || view! {
				<Title text="Entries" />
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