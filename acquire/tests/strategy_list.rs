mod common;
use common::{init, list, feed_strat_name};
use acquire::{
	strategy::Strategy,
	mock::MockStrat, RunError, strategy_list::RunIdError
};
use entities::prelude::*;

///A simple test that can copy/pasted to be the basis of other tests
#[tokio::test]
async fn basic() -> Result<(), RunError> {
	let db = init().await?;
	let strat = MockStrat::default();
	let strat_name = strat.name();
	let strats = list(strat);
	let feed = feed_strat_name("ok", strat_name, &db).await?;
	
	let fetch = strats.run(&db, feed).await?;
	
	assert_eq!(fetch.status, fetch::Status::Success);
	
	Ok(())
}

///run_id works
#[tokio::test]
async fn id() -> Result<(), RunIdError> {
	let db = init().await?;
	let strat = MockStrat::default();
	let strat_name = strat.name();
	let strats = list(strat);
	let feed = feed_strat_name("ok", strat_name, &db).await?;
	
	let fetch = strats.run_id(feed.id, &db).await?;
	
	assert_eq!(fetch.status, fetch::Status::Success);
	
	Ok(())
}