pub mod app;

pub mod feeds;
pub mod fetch;
pub mod entry;
pub mod tag;
pub mod strategies;

pub mod table;
pub mod utils;


#[cfg(feature="ssr")]
pub fn extend(router: axum::routing::Router) -> axum::routing::Router {
	use acquire::*;
	use axum::Extension;
	
	router
		.layer(Extension( {
			let mut list = StrategyList::new();
			list.add(mock::MockStrat::default());
			list.add(yt_dlp::YtDlpStrategy::default());
			list
		} ))
		.layer(Extension(
			batch_tracker::BatchTracker::default()
		))
}