use tokio::sync::{mpsc, broadcast};
use entities::prelude::fetch;
use sea_orm::DatabaseConnection as Db;
use crate::{StrategyList, strategy_list::RunIdError};


pub type FetchResult = Result<fetch::Model, RunIdError>;

struct Batch {
	total: usize,
	finished: Vec<FetchResult>,
}

impl Batch {
	pub fn new(total: usize) -> Self {
		Self {
			total,
			finished: Vec::new(),
		}
	}
	
	pub fn add_done(&mut self, fetch_result: FetchResult) {
		self.finished.push(fetch_result);
	}
	
	pub fn is_done(&self) -> bool {
		self.total == self.finished.len()
	}
	
	pub fn status(&self) -> BatchStatusUpdate {
		BatchStatusUpdate {
			total: self.total,
			done: self.finished.len(),
		}
	}
}

#[non_exhaustive]
#[derive(Debug, Clone,Copy,PartialEq,Eq)]
pub struct BatchStatusUpdate {
	pub total: usize,
	pub done: usize,
}

enum BatchMessage {
	Done(FetchResult)
}

//TODO: this fetches every feed again, while making the list of ids requires fetching all of them in the first place
/**
Fetches all the feeds with the given ids in parallel (every feed gets spawned a new task).

The returned results are probably in a different order then the feed ids. Check fetch.feed_id to get the corresponding feed.
*/
pub async fn fetch_batch(
	feeds: Vec<i32>,
	updates: broadcast::Sender<BatchStatusUpdate>,
	strats: StrategyList,
	db: Db
) -> Vec<FetchResult> {
	let mut batch = Batch::new(feeds.len());
	let (send, mut receive) = mpsc::channel(16);
	
	for id in feeds {
		emit_fetch(id, send.clone(), &strats, db.clone());
	}
	
	loop {
		let maybe_mes = receive.recv().await;
		let Some(mes) = maybe_mes else {
			panic!("No messages left, not sure what to do");
		};
		
		match mes {
			BatchMessage::Done(result) => {
				batch.add_done(result);
			}
		}
		
		//Don't care if nobody's listening
		let _ = updates.send(batch.status());
		
		if batch.is_done() {
			break;
		}
	}
	
	batch.finished
}

///Spawns a new task that fetches the feed, while sending update(s) along the channel
fn emit_fetch(feed_id: i32, channel: mpsc::Sender<BatchMessage>, strats: &StrategyList, db: Db) {
	let strats = strats.clone();
	tokio::spawn(async move {
		let result = strats.run_id(feed_id, &db).await;
		channel.send(BatchMessage::Done(result)).await.expect("someone should be listening");
	});
		
}