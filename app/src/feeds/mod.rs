use leptos::*;
use leptos_router::{Route, Outlet};
use leptos_meta::Title;


pub mod new;
pub mod details;
pub mod search;


#[component(transparent)]
pub fn FeedRoutes() -> impl IntoView {
	view! {
		<Route path="/feed" view=Outlet>
			<Route path="" view= || view! {
				<Title text="Feeds" />
				<main>
					<Outlet/>
				</main>
			}>
				<Route path="" view=search::Search />
				<Route path="/new" view=new::FeedCreator />
			</Route>
			<details::Routes />
		</Route>
	}
}