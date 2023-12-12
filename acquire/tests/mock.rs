mod common;
use common::{init, feed};
use sea_orm::DbErr;
use acquire::{strategy, mock::MockStrat};
use entities::*;

#[tokio::test]
async fn basic() -> Result<(), DbErr> {
	let db = init().await?;
	let strat = MockStrat::default();
	let feed = feed("ok", &strat, &db).await?;
	
	let fetch = strategy::run_strategy(db, &feed, &strat).await?;
	
	assert_eq!(fetch.status, fetch::Status::Success);
	
	Ok(())
}