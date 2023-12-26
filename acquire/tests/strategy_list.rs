mod common;
use common::{init, feed_strat_name};
use acquire::{
	strategy::Strategy,
	mock::MockStrat, StrategyList, RunError
};
use entities::prelude::*;

fn list(strat: impl Strategy + Send + Sync + 'static) -> StrategyList {
	let mut list = StrategyList::new();
	list.add(strat);
	list
}

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