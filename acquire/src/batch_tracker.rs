use thiserror::Error;
use tokio::{
	sync::{broadcast, RwLock},
	task::{JoinHandle, JoinError}
};
use sea_orm::DatabaseConnection as Db;
use tracing::Instrument;
use std::sync::Arc;
use crate::{
	StrategyList,
	batch::{Batch, FetchResult, BatchStatusUpdate, fetch_batch, Listener}
};

type Receiver = broadcast::Receiver<BatchStatusUpdate>;
type Sync<Item> = Arc<RwLock<Item>>;


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
	pub status: Sync<Batch>,
	pub listener: Receiver,
	///handle of the fetching task
	pub fetch_handle: Sync<Option<JoinHandle<()>>>,
}

impl TrackedBatch {
	///`log_identifier` gets added to to the [tracing::Span] of the underlying [Future](std::future::Future) as "id"
	pub fn new_fetch(
		feeds: Vec<i32>,
		strats: StrategyList,
		db: Db,
		log_identifier: impl tracing::Value,
	) -> Self {
		let (send, recv) = broadcast::channel(16);
		let listener = BroadcastListener::from_sender(send);
		
		let (batch, future) = fetch_batch(feeds, listener, strats, db);
		let future = future.instrument(tracing::info_span!("tracked batch", id=log_identifier));
		
		let fetch_handle = tokio::spawn(future);
		
		Self {
			status: batch,
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

#[derive(Debug, Error)]
#[error("Could not find batch at index {0}")]
pub struct BatchNotFoundError(usize);

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
		
		//Scope to reduce lock time
		{
			let mut batches_lock = self.batches.write().await;
			
			let id = batches_lock.len();
			let tracker = TrackedBatch::new_fetch(feeds, strats, db, id);
			batches_lock.push(tracker);
			id
		}
	}
	
	pub async fn get_status(&self, index: usize) -> Result<Sync<Batch>, BatchNotFoundError> {
		let lock = self.batches.read().await;
		let Some(batch) = lock.get(index) else {
			return Err(BatchNotFoundError(index));
		};
		Ok(batch.status.clone())
	}
	
	pub async fn subscribe(&self, index: usize) -> Result<broadcast::Receiver<BatchStatusUpdate>, BatchNotFoundError> {
		let lock = self.batches.read().await;
		let Some(batch) = lock.get(index) else {
			return Err(BatchNotFoundError(index));
		};
		Ok(batch.listener.resubscribe())
	}
	
	pub async fn await_fetch(&self, index: usize) -> Result<(), AwaitFetchError> {
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
		
		handle.await?;
		
		Ok(())
	}
}