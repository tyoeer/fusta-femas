use thiserror::Error;
use tokio::{sync::{broadcast, RwLock}, task::{JoinHandle, JoinError}};
use sea_orm::DatabaseConnection as Db;
use std::{sync::Arc, ops::DerefMut};
use crate::{StrategyList, batch::{FetchResult, BatchStatusUpdate, fetch_batch, Listener}};

type Receiver = broadcast::Receiver<BatchStatusUpdate>;
type Sync<Item> = Arc<RwLock<Item>>;

#[derive(Default)]
pub struct TrackingListener {
	status: Sync<BatchStatus>
}

impl TrackingListener {
	pub fn get_status(&self) -> Sync<BatchStatus> {
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



#[non_exhaustive]
#[derive(Debug)]
pub struct TrackedBatch {
	pub status: Sync<BatchStatus>,
	pub listener: Receiver,
	///handle of the fetching task
	pub fetch_handle: Sync<Option<JoinHandle<Vec<FetchResult>>>>,
}

impl TrackedBatch {
	pub fn new_fetch(
		feeds: Vec<i32>,
		strats: StrategyList,
		db: Db,
	) -> Self {
		let tl = TrackingListener::default();
		let status = tl.get_status();
		let (send, recv) = broadcast::channel(16);
		let bl = BroadcastListener::from_sender(send);
		
		let listener = (tl, bl);
		
		let fetch_handle = tokio::spawn(fetch_batch(feeds, listener, strats, db));
		
		Self {
			status,
			listener: recv,
			fetch_handle: Arc::new(RwLock::new(Some(fetch_handle))),
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

#[derive(Debug,Error)]
pub enum AwaitFetchError {
	#[error("Could not find batch at index {0}")]
	NotFound(usize),
	#[error(transparent)]
	JoinError(#[from] JoinError),
	#[error("There's no JoinHandle to await, presumably because it is already being awaited somewhere else")]
	NoJoinHandle,
}

#[derive(Default, Debug, Clone)]
pub struct BatchTracker {
	batches: Sync<Vec<TrackedBatch>>,
}

impl BatchTracker {
	pub async fn queue_fetches(&self, feeds: Vec<i32>, db: Db, strats: StrategyList) -> usize {
		let tracker = TrackedBatch::new_fetch(feeds, strats, db);
		
		//Scope to reduce lock time
		{
			let mut batches_lock = self.batches.write().await;
			batches_lock.push(tracker);
			batches_lock.len() - 1
		}
	}
	
	pub async fn get_status(&self, index: usize) -> Result<Sync<BatchStatus>, GetStatusError> {
		let lock = self.batches.read().await;
		let Some(batch) = lock.get(index) else {
			return Err(GetStatusError::NotFound(index));
		};
		Ok(batch.status.clone())
	}
	
	///Ok(None) if it has already been awaited and its results are no longer here
	pub async fn await_fetch(&self, index: usize) -> Result<Option<Vec<FetchResult>>, AwaitFetchError> {
		//Scope to reduce lock time
		let synced_maybe_handle = {
			let lock = self.batches.read().await;
			let Some(batch) = lock.get(index) else {
				return Err(AwaitFetchError::NotFound(index));
			};
			batch.fetch_handle.clone()
		};
		
		//Scope to reduce lock time
		let maybe_handle = {
			let mut lock = synced_maybe_handle.write().await;
			lock.take()
		};
		
		let Some(handle) = maybe_handle else {
			return Err(AwaitFetchError::NoJoinHandle);
		};
		
		let result = handle.await?;
		Ok(Some(result))
	}
}