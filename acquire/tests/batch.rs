mod common;
use common::{init, single_strat_list, cmd_strats, feed_strat_name};

use std::collections::HashSet;
use acquire::{
	strategy::Strategy,
	mock::{MockStrat, FetchCommand}, 
	RunError,
	batch::{fetch_batch, BatchStatusUpdate},
	batch_tracker::{BatchTracker, BroadcastListener}
};
use entities::prelude::*;
use sea_orm::{ModelTrait, PaginatorTrait};
use tokio::sync::broadcast;

const CMD_STRAT: &str = "command strat";

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
	
	let (batch_sync, future) = fetch_batch(vec![feed1.id, feed2.id], listener, strats, db.clone());
	future.await;
	
	assert_eq!(1, feed1.find_related(fetch::Entity).count(&db).await? );
	assert_eq!(1, feed2.find_related(fetch::Entity).count(&db).await? );
	assert_eq!(2, recv.len());
	{ // Scope to reduce lock time
		let batch_lock = batch_sync.read().await;
		assert_eq!(2, batch_lock.finished.len());
	}
	
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
	
	let (batch_sync, future) = fetch_batch(ids.clone(), listener, strats, db.clone());
	future.await;
	
	let batch_lock = batch_sync.read().await;
	
	assert_eq!(ids.len(), batch_lock.finished.len());
	
	let id_set = ids.iter().cloned().collect::<HashSet<i32>>();
	let fetched_ids = batch_lock.finished.iter()
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

///Sent broadcast updates look good
#[tokio::test]
async fn broadcast_listener_updates() -> Result<(), anyhow::Error> {
	let db = init().await?;
	let strat = MockStrat::default();
	let strat_name = strat.name();
	let strats = single_strat_list(strat);
	
	let feed1 = feed_strat_name("ok", strat_name, &db).await?;
	let feed2 = feed_strat_name("ok", strat_name, &db).await?;
	
	let (mut recv, listener) = listener();
	
	let (_batch_sync, future) = fetch_batch(vec![feed1.id, feed2.id], listener, strats, db.clone());
	
	future.await;
	
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

///Things keep looking good when using the tracker
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
	
	let status_sync = tracker.get_status(index).await?;
	let status_lock = status_sync.read().await;
	
	assert_eq!(2, status_lock.total);
	assert_eq!(2, status_lock.finished.len());
	
	assert_eq!(1, feed1.find_related(fetch::Entity).count(&db).await? );
	assert_eq!(1, feed2.find_related(fetch::Entity).count(&db).await? );
	
	Ok(())
}

///The Batch looks good throughout the process
#[tokio::test]
#[tracing::instrument]
async fn batch_status() -> Result<(), anyhow::Error> {
	let db = init().await?;
	let (cmd, strats) = cmd_strats();
	
	let feed1 = feed_strat_name("ok", CMD_STRAT, &db).await?;
	let feed2 = feed_strat_name("ok", CMD_STRAT, &db).await?;
	
	let (mut recv, listener) = listener();
	
	let (batch_sync, future) = fetch_batch(vec![feed1.id, feed2.id], listener, strats, db.clone());
	let join_handle = tokio::spawn(future);
	
	cmd.send(FetchCommand::Fetch(feed1.id))?;
	cmd.send(FetchCommand::Parse(feed1.id))?;
	//wait for it to be processed
	recv.recv().await?;
	
	{ //Scope to reduce lock time
		let lock = batch_sync.read().await;
		assert_eq!(2, lock.total);
		assert_eq!(1, lock.finished.len());
	}
	
	cmd.send(FetchCommand::Fetch(feed2.id))?;
	cmd.send(FetchCommand::Parse(feed2.id))?;
	//wait for it to be processed
	recv.recv().await?;
	
	{ //Scope to reduce lock times
		let lock = batch_sync.read().await;
		assert_eq!(2, lock.total);
		assert_eq!(2, lock.finished.len());
	}
	
	//just to be sure
	join_handle.await?;
	
	Ok(())
}