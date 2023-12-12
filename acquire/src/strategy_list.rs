use std::sync::Arc;
use super::strategy::*;
use entities::*;

#[derive(thiserror::Error,Debug)]
pub enum StrategyError {
	#[error("Database error")]
	Db(#[from] sea_orm::DbErr),
	#[error("Did not find strategy \"{0}\"")]
	NotFound(String),
}

#[derive(Default,Clone)]
pub struct StrategyList {
	list: Vec<Arc<dyn Strategy + Send + Sync>>
}

impl StrategyList {
	pub fn new() -> Self {
		Self::default()
	}
	
	pub fn add(&mut self, strat: impl Strategy + Send + 'static) {
		self.list.push(Arc::new(strat));
	}
	
	pub async fn run(&self, conn: sea_orm::DatabaseConnection, feed: feed::Model) -> Result<fetch::Model, StrategyError> {
		let strat = self.list.iter().find(|s| s.name()==feed.strategy);
		match strat {
			None => Err(StrategyError::NotFound(feed.strategy)),
			Some(strat) => {
				Ok(run_strategy(conn, &feed, strat.as_ref()).await?)
			}
		}
	}
	
	pub fn iter_strats(&self) -> impl Iterator<Item = &(dyn Strategy + Send + Sync)> {
		self.list.iter().map(|s| s.as_ref())
	}
}

impl std::fmt::Debug for StrategyList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let names = self.iter_strats().map(|s| s.name()).collect::<String>();
		f.debug_struct("StrategyList").field("names", &names).finish()
	}
}