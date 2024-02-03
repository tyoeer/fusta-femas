use std::sync::Arc;
use super::strategy::*;
use entities::prelude::*;
use sea_orm::EntityTrait;

#[derive(thiserror::Error,Debug)]
pub enum RunError {
	#[error("Database error")]
	Db(#[from] sea_orm::DbErr),
	#[error(transparent)]
	StrategyNotFound(#[from] NotFoundError),
}

#[derive(thiserror::Error,Debug)]
pub enum RunIdError {
	#[error("Database error")]
	Db(#[from] sea_orm::DbErr),
	#[error("Could not find feed with id \"{0}\"")]
	NoSuchFeed(i32),
	#[error(transparent)]
	StrategyNotFound(#[from] NotFoundError),
}

impl From<RunError> for RunIdError {
	fn from(run_error: RunError) -> Self {
		match run_error {
			RunError::Db(db) => RunIdError::Db(db),
			RunError::StrategyNotFound(nfe) => RunIdError::StrategyNotFound(nfe),
		}
	}
}

#[derive(thiserror::Error,Debug)]
#[error("Could not find strategy \"{0}\"")]
pub struct NotFoundError(String);


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
	pub fn add_from_container(&mut self, strat: impl Into<Arc<dyn Strategy + Send + Sync>>) {
		self.list.push(strat.into());
	}
	
	pub fn get_by_name(&self, name: &str) -> Result<&Arc<dyn Strategy + Send + Sync>, NotFoundError> {
		self.list.iter()
			.find(|s| s.name()==name)
			.ok_or_else(|| NotFoundError(name.to_owned()))
	}
	
	pub async fn run(&self, conn: &sea_orm::DatabaseConnection, feed: feed::Model) -> Result<fetch::Model, RunError> {
		let strat = self.get_by_name(&feed.strategy)?;
		let fetch = run_strategy(conn, &feed, strat.as_ref()).await?;
		Ok(fetch)
	}
	
	pub async fn run_id(&self, feed_id: i32, db: &sea_orm::DatabaseConnection) -> Result<fetch::Model ,RunIdError> {
		let maybe_feed: Option<feed::Model> = feed::Entity::find_by_id(feed_id)
			.one(db)
			.await?;
		let Some(feed) = maybe_feed else {
			return Err(RunIdError::NoSuchFeed(feed_id))
		};
		
		let fetch = self.run(db, feed).await?;
		
		Ok(fetch)
	}
	
	pub fn iter_strats(&self) -> impl Iterator<Item = &(dyn Strategy + Send + Sync)> {
		self.list.iter().map(|s| s.as_ref())
	}
}

impl std::fmt::Debug for StrategyList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let names = self.iter_strats().map(|s| s.name()).collect::<Vec<_>>();
		f.debug_struct("StrategyList").field("names", &names).finish()
	}
}