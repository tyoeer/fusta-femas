use tokio::{sync::{broadcast, RwLock}, task::JoinHandle};
use sea_orm::DatabaseConnection as Db;
use std::{sync::Arc, ops::DerefMut};
use crate::{StrategyList, batch::{FetchResult, BatchStatusUpdate, fetch_batch, Listener}};

type ListenerAlias = broadcast::Receiver<BatchStatusUpdate>;

pub struct BroadcastListener {
	sender: broadcast::Sender<BatchStatusUpdate>,
}

impl BroadcastListener {
	pub fn from_sender(sender: broadcast::Sender<BatchStatusUpdate>) -> Self {
		Self {
			sender,
		}
	}
}

impl Listener for BroadcastListener {
	fn fetch_finished(&mut self, update: BatchStatusUpdate) {
		//don't care of nobody is listening
		let _ = self.sender.send(update);
	}
}

async fn update_status_loop(status: Arc<RwLock<BatchStatus>>, mut listener: ListenerAlias) {
	loop {
		let Ok(update) = listener.recv().await else {
			//Nobody is sending updates, we can stop
			break;
		};
		let mut lock = status.write().await;
		let _old = std::mem::replace(lock.deref_mut(), BatchStatus::InProgress(update));
	}
}

#[non_exhaustive]
#[derive(Debug)]
pub struct TrackedBatch {
	pub status: Arc<RwLock<BatchStatus>>,
	pub listener: ListenerAlias,
	///Tokio task that keeps updating status with the newest message from listener
	pub updater: JoinHandle<()>,
}

impl TrackedBatch {
	pub fn from_listener(recv: ListenerAlias) -> Self {
		let status = Arc::new(RwLock::new(BatchStatus::Starting));
		let updater = tokio::spawn(update_status_loop(status.clone(), recv.resubscribe()));
		
		Self {
			status,
			listener: recv,
			updater,
		}
	}
}

#[derive(Debug)]
pub enum BatchStatus {
	Starting,
	InProgress(BatchStatusUpdate),
	Finished(Vec<FetchResult>)
}

#[derive(Default, Debug, Clone)]
pub struct BatchTracker {
	batches: Arc<RwLock<Vec<TrackedBatch>>>,
}

impl BatchTracker {
	pub async fn queue_fetches(&self, feeds: Vec<i32>, db: Db, strats: StrategyList) -> JoinHandle<Vec<FetchResult>> {
		let (send, recv) = broadcast::channel(16);
		
		let listener = BroadcastListener::from_sender(send);
		let tracker = TrackedBatch::from_listener(recv);
		
		//Scope to reduce lock time
		{
			let mut batches_lock = self.batches.write().await;
			batches_lock.push(tracker);
		}
		
		tokio::spawn(fetch_batch(feeds, listener, strats, db))	
	}
}