use leptos_router::*;
use leptos::*;

pub mod feeds;

#[component(transparent)]
pub fn BackendRoutes(cx: Scope) -> impl IntoView {
	view! { cx,
		<Route path="/backend" view=Outlet>
			<Route path="/feeds" view=feeds::Feeds />
			<Route path="/strats" view=Strategies />
		</Route>
	}
}

#[cfg(feature="ssr")]
pub fn layer(router: axum::routing::Router) -> axum::routing::Router {
    use backend_core::*;

	let mut list = StrategyList::new();
	list.add(strategy::MockStrat);
	router.layer(axum::Extension(list))
}

#[server(GetStrats, "/api")]
pub async fn get_strategies(cx: Scope) -> Result<Vec<String>, ServerFnError> {
	use backend_core::strategy_list::StrategyList;
	use axum::*;
	
	leptos_axum::extract(cx, |Extension(strats): Extension<StrategyList>| async move {
		strats.iter_strats().map(|s| s.name().to_owned()).collect::<Vec<String>>()
	}).await.map_err(|e| {
		ServerFnError::ServerError(format!("{:?}",e))
	})
}

#[component]
pub fn Strategies(cx: Scope) -> impl IntoView {
	view! {cx,
		<Await future=get_strategies bind:strats>
			<ul>
				{
					strats.clone().map(|vec| {
						vec.into_iter()
							.map(|e| view! {cx, <li>{e}</li>})
							.collect::<Vec<_>>()
					})
				}
			</ul>
		</Await>
	}
}