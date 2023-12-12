pub mod app;
pub mod error_template;

pub mod feeds;
pub mod fetch;
pub mod strategies;

pub mod table;
pub mod utils;


#[cfg(feature="ssr")]
pub fn extend(router: axum::routing::Router) -> axum::routing::Router {
	
	router.layer(axum::Extension({
		use acquire::*;
		
		let mut list = StrategyList::new();
		list.add(mock::MockStrat::default());
		list.add(yt_dlp::YtDlpStrategy::default());
		list
	}))
}