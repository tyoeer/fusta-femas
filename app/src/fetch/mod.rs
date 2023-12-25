use leptos::*;
use leptos_router::{Route, Outlet};
use leptos_meta::Title;


pub mod details;
pub mod search;


#[component(transparent)]
pub fn Routes() -> impl IntoView {
	view! {
		<Route path="/fetch" view=Outlet>
			<Route path="" view= || view! {
				<Title text="Fetch" />
				<main>
					<Outlet/>
				</main>
			}>
				<Route path="" view=|| view! {
					TODO
				} />
			</Route>
			<details::Routes />
		</Route>
	}
}