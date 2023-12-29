use thiserror::Error;
use tokio::{sync::{broadcast, RwLock}, task::JoinHandle};
use sea_orm::DatabaseConnection as Db;
use std::{sync::Arc, ops::DerefMut};
use crate::{StrategyList, batch::{FetchResult, BatchStatusUpdate, fetch_batch, Listener}};

type ListenerAlias = broadcast::Receiver<BatchStatusUpdate>;


#[derive(Default)]
pub struct TrackingListener {
	status: Arc<RwLock<BatchStatus>>
}

impl TrackingListener {
	pub fn get_status(&self) -> Arc<RwLock<BatchStatus>> {
		self.status.clone()
	}
}

impl Listener for TrackingListener {
	async fn fetch_finished(&mut self, update: BatchStatusUpdate) {
		let mut lock = self.status.write().await; 
		let _old = std::mem::replace(lock.deref_mut(), BatchStatus::InProgress(update));
	}
}


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
	async fn fetch_finished(&mut self, update: BatchStatusUpdate) {
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
	pub fn create() -> (Self, impl Listener) {
		let tl = TrackingListener::default();
		let status = tl.get_status();
		let (send, recv) = broadcast::channel(16);
		let bl = BroadcastListener::from_sender(send);
		
		let this = Self {
			status,
			listener: recv,
			updater: tokio::spawn(async {}),
		};
		
		(this, (tl, bl))
	}
	
	#[deprecated]
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

#[derive(Debug, Default)]
pub enum BatchStatus {
	#[default]
	Starting,
	InProgress(BatchStatusUpdate),
	Finished(Vec<FetchResult>)
}

#[derive(Debug,Error)]
pub enum GetStatusError {
	#[error("Could not find batch at index {0}")]
	NotFound(usize)
}

#[derive(Default, Debug, Clone)]
pub struct BatchTracker {
	batches: Arc<RwLock<Vec<TrackedBatch>>>,
}

impl BatchTracker {
	pub async fn queue_fetches(&self, feeds: Vec<i32>, db: Db, strats: StrategyList) -> usize {
		let (tracker, listener) = TrackedBatch::create();
		
		//Scope to reduce lock time
		let index = {
			let mut batches_lock = self.batches.write().await;
			batches_lock.push(tracker);
			batches_lock.len() - 1
		};
		
		tokio::spawn(fetch_batch(feeds, listener, strats, db));
		
		index
	}
	
	pub async fn get_status(&self, index: usize) -> Result<Arc<RwLock<BatchStatus>>, GetStatusError> {
		let lock = self.batches.read().await;
		let Some(batch) = lock.get(index) else {
			return Err(GetStatusError::NotFound(index));
		};
		Ok(batch.status.clone())
	}
}