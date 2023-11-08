use cfg_if::cfg_if;
use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use backend::BackendRoutes;

cfg_if! { if #[cfg(feature = "ssr")] {
	#[derive(Clone)]
	pub struct AppState {
		pub leptos_options: leptos::LeptosOptions,
		pub conn: sea_orm::DatabaseConnection,
	}

	impl axum::extract::FromRef<AppState> for LeptosOptions {
		fn from_ref(app_state: &AppState) -> LeptosOptions {
			app_state.leptos_options.clone()
		}
	}
}}

#[component]
pub fn App() -> impl IntoView {
	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();
	let (_is_routing, set_is_routing) = create_signal(false);
	
	view! {
		// id=leptos means cargo-leptos will hot-reload this stylesheet
		<Stylesheet id="leptos" href="/pkg/fusta-femas.css"/>
		<Title text="Fusta Femas"/>
		
		// content for this welcome page
		<Router set_is_routing fallback=|| {
			let mut outside_errors = Errors::default();
			outside_errors.insert_with_default_key(AppError::NotFound);
			view! {
				<ErrorTemplate outside_errors/>
			}
			.into_view()
		}>
			// Default style makes it very quickyl move the page up and down
			// <RoutingProgress _is_routing />
			<Nav/>
			<main>
				<Routes>
					<Route path="" view=HomePage />
					<BackendRoutes/>
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
