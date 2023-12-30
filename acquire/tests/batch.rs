mod common;
use std::{collections::HashSet, ops::Deref};

use common::{init, single_strat_list, feed_strat_name};
use acquire::{
	strategy::Strategy,
	mock::MockStrat, 
	RunError,
	batch::{fetch_batch, BatchStatusUpdate}, batch_tracker::{BatchTracker, BroadcastListener, BatchStatus}
};
use entities::prelude::*;
use sea_orm::{ModelTrait, PaginatorTrait};
use tokio::sync::broadcast;


fn listener() -> (broadcast::Receiver<BatchStatusUpdate>, BroadcastListener) {
	let (send, recv) = broadcast::channel(256);
	
	(recv, BroadcastListener::from_sender(send))
}


///A simple test that can copy/pasted to be the basis of other tests
#[tokio::test]
async fn basic() -> Result<(), RunError> {
	let db = init().await?;
	let strat = MockStrat::default();
	let strat_name = strat.name();
	let strats = single_strat_list(strat);
	
	let feed1 = feed_strat_name("ok", strat_name, &db).await?;
	let feed2 = feed_strat_name("ok", strat_name, &db).await?;
	
	let (recv, listener) = listener();
	
	fetch_batch(vec![feed1.id, feed2.id], listener, strats, db.clone()).await;
	
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
	let strats = single_strat_list(strat);
	
	let feed1 = feed_strat_name("ok", strat_name, &db).await?;
	let feed2 = feed_strat_name("ok", strat_name, &db).await?;
	
	let ids = vec![feed1.id, feed2.id];
	
	let (recv, listener) = listener();
	std::mem::drop(recv); // don't care
	
	let results = fetch_batch(ids.clone(), listener, strats, db.clone()).await;
	
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
	let strats = single_strat_list(strat);
	
	let feed1 = feed_strat_name("ok", strat_name, &db).await?;
	let feed2 = feed_strat_name("ok", strat_name, &db).await?;
	
	let (mut recv, listener) = listener();
	
	fetch_batch(vec![feed1.id, feed2.id], listener, strats, db.clone()).await;
	
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

///Sent updates look good
#[tokio::test]
async fn tracked() -> Result<(), anyhow::Error> {
	let db = init().await?;
	let strat = MockStrat::default();
	let strat_name = strat.name();
	let strats = single_strat_list(strat);
	
	let feed1 = feed_strat_name("ok", strat_name, &db).await?;
	let feed2 = feed_strat_name("ok", strat_name, &db).await?;
	
	let tracker = BatchTracker::default();
	
	let index = tracker.queue_fetches(vec![feed1.id, feed2.id], db.clone(), strats).await;
	tracker.await_fetch(index).await?;
	
	let status = tracker.get_status(index).await?;
	let lock = status.read().await;
	
	match lock.deref() {
		BatchStatus::InProgress(status) => {
			assert_eq!(2, status.done);
			assert_eq!(2, status.total);
		}
		batch => panic!("BatchStatus not good: {batch:?}"),
	}
	
	assert_eq!(1, feed1.find_related(fetch::Entity).count(&db).await? );
	assert_eq!(1, feed2.find_related(fetch::Entity).count(&db).await? );
	
	Ok(())
}