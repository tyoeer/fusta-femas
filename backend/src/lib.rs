use leptos::*;
use leptos_router::*;

pub mod feeds;

#[component(transparent)]
pub fn BackendRoutes(cx: Scope) -> impl IntoView {
	view! { cx,
		<Route path="/feeds" view=feeds::Feeds />
	}
}