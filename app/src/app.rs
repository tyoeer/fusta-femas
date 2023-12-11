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
		<ErrorBoundary fallback=|errors| view!{
			<main>
				<ErrorsView errors/>
			</main>
		} >
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
				<div class="global_section">
					<Routes>
						<Route path="" view=HomePage />
						<crate::feeds::FeedRoutes />
						<crate::fetch::Routes />
						<Route path="/strats" view=crate::strategies::Strategies />
					</Routes>
				</div>
			</Router>
		</ErrorBoundary>
	}
}

#[component]
fn Nav() -> impl IntoView {
	view! {
		<nav class="global">
			<A href="">Home</A>
			<A href="feed">Feeds</A>
			<A href="strats">Strategies</A>
		</nav>
	}
}

#[component]
pub fn ErrorsView(errors: RwSignal<Errors>) -> impl IntoView {
	view! {
		<h1>ERROR</h1>
		<ul>
			<For
				each = move || errors.get().into_iter()
				key = |err| err.0.clone()
				let:err
			>
				<li>
					{err.1.to_string()}
				</li>
			</For>
		</ul>
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
