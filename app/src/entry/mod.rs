use leptos::*;
use leptos_router::{Route, Redirect, A, Outlet};
use entities::*;
use crate::table;
use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;


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
			</Route>
		</Route>
	}
}