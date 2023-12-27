mod common;
use std::collections::HashSet;

use common::{init, list, feed_strat_name};
use acquire::{
	strategy::Strategy,
	mock::MockStrat, 
	RunError,
	batch::fetch_batch
};
use entities::prelude::*;
use sea_orm::{ModelTrait, PaginatorTrait};
use tokio::sync::broadcast;



///A simple test that can copy/pasted to be the basis of other tests
#[tokio::test]
async fn basic() -> Result<(), RunError> {
	let db = init().await?;
	let strat = MockStrat::default();
	let strat_name = strat.name();
	let strats = list(strat);
	
	let feed1 = feed_strat_name("ok", strat_name, &db).await?;
	let feed2 = feed_strat_name("ok", strat_name, &db).await?;
	
	let (send, recv) = broadcast::channel(256);
	
	fetch_batch(vec![feed1.id, feed2.id], send, strats, db.clone()).await;
	
	assert_eq!(1, feed1.find_related(fetch::Entity).count(&db).await? );
	assert_eq!(1, feed2.find_related(fetch::Entity).count(&db).await? );
	assert_eq!(2, recv.len());
	
	Ok(())
}

///The generated results look good
#[tokio::test]
async fn results() -> Result<(), RunError> {
	let db = init().await?;
	let strat = MockStrat::default();
	let strat_name = strat.name();
	let strats = list(strat);
	
	let feed1 = feed_strat_name("ok", strat_name, &db).await?;
	let feed2 = feed_strat_name("ok", strat_name, &db).await?;
	
	let ids = vec![feed1.id, feed2.id];
	
	let (send, recv) = broadcast::channel(1);
	std::mem::drop(recv); // not caring
	
	let results = fetch_batch(ids.clone(), send, strats, db.clone()).await;
	
	assert_eq!(ids.len(), results.len());
	
	let id_set = ids.iter().cloned().collect::<HashSet<i32>>();
	let fetched_ids = results.iter()
		.map(|fetch_res| {
			match fetch_res {
				Ok(fetch) => fetch.feed_id,
				Err(err) => {
					tracing::error!(?err);
					panic!("Internal error running a fetch");
				},
			}
		})
		.collect::<HashSet<i32>>();
	
	assert_eq!(id_set, fetched_ids);
	
	Ok(())
}

///Sent updates look good
#[tokio::test]
async fn updates() -> Result<(), anyhow::Error> {
	let db = init().await?;
	let strat = MockStrat::default();
	let strat_name = strat.name();
	let strats = list(strat);
	
	let feed1 = feed_strat_name("ok", strat_name, &db).await?;
	let feed2 = feed_strat_name("ok", strat_name, &db).await?;
	
	let (send, mut recv) = broadcast::channel(256);
	
	fetch_batch(vec![feed1.id, feed2.id], send, strats, db.clone()).await;
	
	assert_eq!(1, feed1.find_related(fetch::Entity).count(&db).await? );
	assert_eq!(1, feed2.find_related(fetch::Entity).count(&db).await? );
	
	let update = recv.recv().await?;
	assert_eq!(2, update.total);
	assert_eq!(1, update.done);
	let update = recv.recv().await?;
	assert_eq!(2, update.total);
	assert_eq!(2, update.done);
	
	Ok(())
}