pub mod app;
pub mod error_template;
pub mod strategies;
pub mod feeds;

pub mod table;
pub mod utils;


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