use tokio::sync::mpsc;
use entities::prelude::fetch;
use sea_orm::DatabaseConnection as Db;
use crate::{StrategyList, strategy_list::RunIdError};


struct Batch {
	total: usize,
	done: usize,
}

impl Batch {
	pub fn new(total: usize) -> Self {
		Self {
			total,
			done: 0,
		}
	}
	
	pub fn add_done(&mut self) {
		self.done += 1;
	}
	
	pub fn is_done(&self) -> bool {
		self.total == self.done
	}
}

enum BatchMessage {
	Done(Result<fetch::Model,RunIdError>)
}

//TODO: this fetches every feed again, while making the list of ids requires fetching all of them in the first place

pub async fn fetch_batch(feeds: Vec<i32>, strats: StrategyList, db: Db) {
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
			BatchMessage::Done(_) => {
				//TODO check result for error
				//TODO do something with failed fetch?
				batch.add_done();
			}
		}
		
		//TODO broadcast update
		
		if batch.is_done() {
			break;
		}
	}
	
	//TODO maybe broadcast final update?
}

///Spawns a new task that fetches the feed, while sending update(s) along the channel
fn emit_fetch(feed_id: i32, channel: mpsc::Sender<BatchMessage>, strats: &StrategyList, db: Db) {
	let strats = strats.clone();
	tokio::spawn(async move {
		let result = strats.run_id(feed_id, &db).await;
		channel.send(BatchMessage::Done(result)).await.expect("someone should be listening");
	});
		
}