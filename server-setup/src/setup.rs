use axum::{
	routing::Router,
	Extension,
};
use acquire::{
	batch_tracker::BatchTracker,
	strategy::Strategy,
	StrategyList,
};

#[derive(Debug, Clone, Default)]
pub struct Setup {
	pub strategies: StrategyList,
}

impl Setup {
	pub fn add_strategy(&mut self, strategy: impl Strategy + Send + 'static) {
		self.strategies.add(strategy);
	}
	pub fn extend(self, router: Router) -> Router {
		router
			.layer(Extension(self.strategies))
			.layer(Extension(BatchTracker::default()))
	}
	
	pub fn extend_fn(self) -> impl FnOnce(Router) -> Router {
		move |router| self.extend(router)
	}
}