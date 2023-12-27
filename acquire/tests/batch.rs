mod common;
use common::{init, list, feed_strat_name};
use acquire::{
	strategy::Strategy,
	mock::MockStrat, 
	RunError,
	batch::fetch_batch
};
use entities::prelude::*;
use sea_orm::{ModelTrait, PaginatorTrait};



///A simple test that can copy/pasted to be the basis of other tests
#[tokio::test]
async fn basic() -> Result<(), RunError> {
	let db = init().await?;
	let strat = MockStrat::default();
	let strat_name = strat.name();
	let strats = list(strat);
	
	let feed1 = feed_strat_name("ok", strat_name, &db).await?;
	let feed2 = feed_strat_name("ok", strat_name, &db).await?;
	
	fetch_batch(vec![feed1.id, feed2.id], strats, db.clone()).await;
	
	assert_eq!(1, feed1.find_related(fetch::Entity).count(&db).await? );
	assert_eq!(1, feed2.find_related(fetch::Entity).count(&db).await? );
	
	Ok(())
}