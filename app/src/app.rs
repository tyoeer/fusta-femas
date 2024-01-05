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
		<Title text="" formatter=|text: String| {
			if text.is_empty() {
				"Fusta Femas".to_owned()
			} else {
				format!("{text} - Fusta Femas")
			}
		}/>
		
		<Router set_is_routing fallback=|| NotFound().into_view()>
			<ErrorBoundary fallback=|errors| view!{
				<main>
					<ErrorsView errors/>
				</main>
			} >
				// Default style makes it very quickly move the page up and down
				// <RoutingProgress _is_routing />
				<Nav/>
				<div class="global_section">
					<Routes>
						<Route path="" view=HomePage />
						<crate::feeds::FeedRoutes />
						<crate::fetch::Routes />
						<crate::fetch::batch::Routes />
						<crate::entry::Routes />
						<Route path="/strats" view=crate::strategies::Strategies />
					</Routes>
				</div>
			</ErrorBoundary>
		</Router>
	}
}

#[component]
fn Nav() -> impl IntoView {
	view! {
		<nav class="global">
			<A href="">Home</A>
			<A href="entry">Entries</A>
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

#[component]
fn NotFound() -> impl IntoView {
	view! {
		<Nav/>
		<main>
			<h1>"404 Not Found"</h1>
			<p> "Did not find the page you're looking for" </p>
		</main>
	}
}

/// Renders the template home page
#[component]
fn HomePage() -> impl IntoView {
	view! {
		<main>
			<h1>"Welcome to Fusta Femas"</h1>
			<crate::fetch::batch::FetchAllButton redirect=true/>
			<crate::fetch::batch::FetchAllButton />
		</main>
	}
}
