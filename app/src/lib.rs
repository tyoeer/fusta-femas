pub mod app;
pub mod error_template;
mod backend;
mod feeds;

#[cfg(feature="ssr")]
pub fn extend(router: axum::routing::Router) -> axum::routing::Router {
	
	router.layer(axum::Extension({
		use acquire::*;
		
		let mut list = StrategyList::new();
		list.add(strategy::MockStrat);
		list.add(yt_dlp::YtDlpStrategy::default());
		list
	}))
}