use leptos::*;
use leptos_router::{Route, Outlet};
use leptos_meta::Title;


pub mod new;
// pub mod details;
pub mod search;


#[component(transparent)]
pub fn Routes() -> impl IntoView {
	view! {
		<Route path="/tag" view=Outlet>
			<Route path="" view= || view! {
				<Title text="Tags" />
				<main>
					<Outlet/>
				</main>
			}>
				<Route path="" view=search::Search />
				<Route path="/new" view=new::TagCreator />
			</Route>
			// <details::Routes />
		</Route>
	}
}