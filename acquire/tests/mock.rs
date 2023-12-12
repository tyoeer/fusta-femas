mod common;
use common::{init, feed};
use sea_orm::DbErr;
use acquire::{
	strategy::{
		self,
		Strategy
	},
	mock::MockStrat
};
use entities::*;

///A simple test that can copy/pasted to be the basis of other tests
#[tokio::test]
async fn basic() -> Result<(), DbErr> {
	let db = init().await?;
	let strat = MockStrat::default();
	let feed = feed("ok", &strat, &db).await?;
	
	let fetch = strategy::run_strategy(db, &feed, &strat).await?;
	
	assert_eq!(fetch.status, fetch::Status::Success);
	
	Ok(())
}

///The created fetch looks good
#[tokio::test]
async fn fetch() -> Result<(), DbErr> {
	let db = init().await?;
	let strat = MockStrat::default();
	let feed = feed("ok", &strat, &db).await?;
	
	let fetch = strategy::run_strategy(db, &feed, &strat).await?;
	
	assert_eq!(fetch.status, fetch::Status::Success);
	assert_eq!(fetch.feed_id, feed.id);
	assert_eq!(fetch.strategy, strat.name());
	assert!(fetch.error.is_none());
	assert!(fetch.content.is_some());
	
	Ok(())
}

///Thinks look alright when a fetch error occurred
#[tokio::test]
async fn fetch_error() -> Result<(), DbErr> {
	let db = init().await?;
	let strat = MockStrat::default();
	let feed = feed("fetch error", &strat, &db).await?;
	
	let fetch = strategy::run_strategy(db, &feed, &strat).await?;
	
	assert_eq!(fetch.status, fetch::Status::FetchError);
	assert!(fetch.error.is_some());
	assert!(fetch.content.is_none());
	
	Ok(())
}

///Thinks look alright when a parse error occurred
#[tokio::test]
async fn parse_error() -> Result<(), DbErr> {
	let db = init().await?;
	let strat = MockStrat::default();
	let feed = feed("parse error", &strat, &db).await?;
	
	let fetch = strategy::run_strategy(db, &feed, &strat).await?;
	
	assert_eq!(fetch.status, fetch::Status::ParseError);
	assert!(fetch.error.is_some());
	assert!(fetch.content.is_some());
	
	Ok(())
}