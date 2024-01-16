use leptos::*;
use leptos_router::{Route, A, Outlet};
use leptos_meta::Title;
use crate::utils;

pub mod new;
pub mod details;
pub mod search;


#[component(transparent)]
pub fn FeedRoutes() -> impl IntoView {
	view! {
		<Route path="/feed" view=Outlet>
			<Route path="" view= || view! {
				<Title text="Feeds" />
				<Sidebar />
				<main>
					<Outlet/>
				</main>
			}>
				<utils::RouteAlias to="search" />
				<Route path="/search" view=search::Search />
				<Route path="/all" view=search::All />
				<Route path="/new" view=new::FeedCreator />
			</Route>
			<details::Routes />
		</Route>
	}
}

#[component]
fn Sidebar() -> impl IntoView {
	view! {
		<nav class="sidebar">
			<ul>
				<li>
					<A href="search">Search</A>
				</li>
				<li>
					<A href="all">All</A>
				</li>
				<li>
					<A href="new">New</A>
				</li>
			</ul>
		</nav>
	}
}