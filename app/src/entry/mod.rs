use leptos::*;
use leptos_router::{Route, Redirect, Outlet};

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
			<Route path="/:id" view=details::Navbar>
				<Route path="" view=|| view! { <Redirect path="about"/> }/>
				<Route path="about" view = || {
					crate::utils::with_id_param(|id| view! {
						<details::About id />
					})
				} />
				<Route path="embedded" view = || {
					crate::utils::with_id_param(|id| view! {
						<details::Embed id />
					})
				} />
				<Route path="fetches" view = || {
					crate::utils::with_id_param(|id| view! {
						<details::Fetches id />
					})
				} />
			</Route>
		</Route>
	}
}