use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;


#[component]
pub fn App() -> impl IntoView {
	// Needed for stylesheet, title, etc.
	provide_meta_context();
	let (_is_routing, set_is_routing) = create_signal(false);
	
	view! {
		// id=leptos means cargo-leptos will hot-reload this stylesheet
		<Stylesheet id="leptos" href="/pkg/fusta-femas.css"/>
		<Title text="Fusta Femas"/>
		
		<Router set_is_routing fallback=|| {
			let mut outside_errors = Errors::default();
			outside_errors.insert_with_default_key(AppError::NotFound);
			view! {
				<ErrorTemplate outside_errors/>
			}
			.into_view()
		}>
			// Default style makes it very quickly move the page up and down
			// <RoutingProgress _is_routing />
			<Nav/>
			<main>
				<Routes>
					<Route path="" view=HomePage />
					<Route path="/backend" view=Outlet>
						<Route path="/feeds" view=crate::feeds::Feeds />
						<Route path="/strats" view=crate::strategies::Strategies />
					</Route>
				</Routes>
			</main>
		</Router>
	}
}

#[component]
fn Nav() -> impl IntoView {
	view! {
		<nav>
			<A href="">Home</A>
			<A href="backend/feeds">Feeds</A>
			<A href="backend/strats">Strategies</A>
		</nav>
	}
}

/// Renders the template home page
#[component]
fn HomePage() -> impl IntoView {
	// Creates a reactive value to update the button
	let (count, set_count) = create_signal(0);
	let on_click = move |_| set_count.update(|count| *count += 1);

	view! {
		<h1>"Welcome to Leptos!"</h1>
		<button on:click=on_click>"Click Me: " {count}</button>
	}
}
