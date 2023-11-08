use leptos_router::*;
use leptos::*;

pub mod feeds;

#[component(transparent)]
pub fn BackendRoutes() -> impl IntoView {
	view! {
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
	list.add(yt_dlp::YtDlpStrategy::default());
	router.layer(axum::Extension(list))
}

#[cfg(feature="ssr")]
pub async fn get_strats() -> Result<backend_core::strategy_list::StrategyList, ServerFnError> {
	use backend_core::strategy_list::StrategyList;
	use axum::*;
	use leptos_axum::extractor;
	
	extractor::<Extension<StrategyList>>().await.map(|ext| ext.0)
}

#[server]
pub async fn get_strategies() -> Result<Vec<String>, ServerFnError> {	
	let strats = get_strats().await?;
	let list = strats.iter_strats().map(|s| s.name().to_owned()).collect::<Vec<String>>();
	Ok(list)
}

#[component]
pub fn Strategies() -> impl IntoView {
	view! {
		<Await future=get_strategies let:strats>
			<ul>
				{
					strats.clone().map(|vec| {
						vec.into_iter()
							.map(|e| view! {<li>{e}</li>})
							.collect::<Vec<_>>()
					})
				}
			</ul>
		</Await>
	}
}