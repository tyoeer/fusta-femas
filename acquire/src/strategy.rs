use std::sync::{Mutex, Arc, MutexGuard};

use entities::prelude::*;
use sea_orm::*;
use tracing::instrument::WithSubscriber;
use tracing_subscriber::{
	fmt,
	fmt::{writer::MutexGuardWriter, MakeWriter}
};

pub struct EntryInfo {
	feed_entry_id: String,
	title: String,
	view_url: String,
	embed_url: Option<String>,
	produced_date: time::Date,
	produced_time: Option<time::Time>,
}

impl EntryInfo {
	pub fn new(feed_entry_id: String, title: String, view_url: String, produced_date: time::Date) -> Self {
		Self {
			feed_entry_id,
			title,
			view_url,
			embed_url: None,
			produced_date,
			produced_time: None,
		}
	}
	
	pub fn produced_time(&mut self, time: time::Time) -> &mut Self {
		self.produced_time = Some(time);
		self
	}
	
	pub fn embed_url(&mut self, embed_url: String) -> &mut Self {
		self.embed_url = Some(embed_url);
		self
	}
}



#[derive(Default, Clone, Debug)]
struct SharedLog {
	log: Arc<Mutex<Vec<u8>>>,
}

impl SharedLog {
	pub fn lock(&self) -> MutexGuard<'_, Vec<u8>> {
		self.log.lock().expect("lock poisoned")
	}
}

impl<'a> MakeWriter<'a> for SharedLog {
	type Writer = MutexGuardWriter<'a, Vec<u8>>;

	fn make_writer(&'a self) -> Self::Writer {
		self.log.make_writer()
	}
}

async fn logged<Output>(task: impl std::future::Future<Output = Output>) -> (String, Output) {
	let shared_log = SharedLog::default();
	let subscriber_share = shared_log.clone();
	let subscriber = fmt()
		.event_format(fmt::format().pretty().with_ansi(false))
		.with_writer(subscriber_share)
		.finish();
	
	let output = task.with_subscriber(subscriber).await;
	
	let log_text = String::from_utf8_lossy(&shared_log.lock()).to_string();
	(log_text, output)
}



///The type itself should serve as a configuration/settings, which is why it should be serializable
#[async_trait::async_trait]
pub trait Strategy: ff_object::traits::DynSer + Send + Sync {
	//&self required to be able to call it in a dyn context
	fn name(&self) -> &'static str;
	async fn fetch(&self, conn: &DatabaseConnection, feed: &feed::Model) -> anyhow::Result<String>;
	async fn parse(&self, data: &str) -> anyhow::Result<Vec<EntryInfo>>;
}


fn error_to_string(err: anyhow::Error) -> String {
	tracing::error!("{err:?}");
	format!("{err:?}")
}

async fn update_entries(conn: &DatabaseConnection, feed: &feed::Model, fetch_id: i32, entries: Vec<EntryInfo>) -> anyhow::Result<()> {
	let feed_entry_ids = entries.iter().map(|e| &e.feed_entry_id);
	let existing = feed.find_related(entry::Entity)
		.filter(entry::Column::FeedEntryId.is_in(feed_entry_ids))
		.all(conn)
		.await?;
	
	let feed_id = feed.id;
	
	conn.transaction::<_,_,DbErr>(|conn| Box::pin(async move {
		for entry in entries {
			let mut model = 'model: {
				for old in &existing {
					if old.feed_entry_id==entry.feed_entry_id {
						break 'model old.clone().into_active_model();
					}
				}
				let mut new = entry::ActiveModel::new();
				new.feed_entry_id = Set(entry.feed_entry_id);
				new.feed_id = Set(feed_id);
				new
			};
			
			model.name = Set(entry.title);
			model.view_url = Set(entry.view_url);
			model.embed_url = Set(entry.embed_url);
			model.produced_date = Set(entry.produced_date.into());
			model.produced_time = Set(entry.produced_time.into());
			let entry_saved = model.save(conn).await?;
			//Shouldn't be an error because we just saved it
			let entry_saved = entry_saved.try_into_model()?;
			
			let mut fetch_entry = fetch_entry::ActiveModel::new();
			fetch_entry.fetch_id = Set(fetch_id);
			fetch_entry.entry_id = Set(entry_saved.id);
			fetch_entry.insert(conn).await?;
		}
		
		Ok(())
	})).await?;
	
	Ok(())
}

///Runs a fetch, but doesn't do the final save, because that still requires the logs from running this function
async fn do_fetch(conn: &DatabaseConnection, feed: &feed::Model, strat: &dyn Strategy) -> Result<fetch::ActiveModel, DbErr> {
	use ActiveValue::Set;
	
	tracing::info!(strategy = strat.name(), ?feed, "Fetching feed");
	
	//setup fetch
	let mut fetch = fetch::ActiveModel::new();
	fetch.feed_id = Set(feed.id);
	fetch.url = Set(feed.url.clone());
	fetch.strategy = Set(strat.name().to_owned());
	
	let fetched = strat.fetch(conn, feed).await;
	
	let data = match fetched {
		Err(err) => {
			fetch.status = Set(fetch::Status::FetchError);
			fetch.error = Set(Some(error_to_string(err)));
			
			return Ok(fetch);
		},
		Ok(data) => data,
	};
	
	fetch.content = Set(Some(data.clone()));
	
	let parsed = strat.parse(&data).await;
	
	let parsed = match parsed {
		Err(err) => {
			fetch.status = Set(fetch::Status::ParseError);
			fetch.error = Set(Some(error_to_string(err)));
			
			return Ok(fetch);
		},
		Ok(parsed) => parsed,
	};
	
	fetch.status = Set(fetch::Status::EntryUpdateError);
	let fetch_inserted = fetch.insert(conn).await?;
	
	let res = update_entries(conn, feed, fetch_inserted.id, parsed).await;
	fetch = fetch_inserted.into_active_model();
	match res {
		Ok(_) => {
			fetch.status = Set(fetch::Status::Success);
		},
		Err(err) => {
			fetch.error = Set(Some(error_to_string(err)));
		}
	}
	
	Ok(fetch)
}

pub async fn run_strategy(conn: &DatabaseConnection, feed: &feed::Model, strat: &dyn Strategy) -> Result<fetch::Model, DbErr> {
	let (log, fetch) = logged(do_fetch(conn, feed, strat)).await;
	let mut fetch = fetch?;
	
	fetch.log = Set(log);
	
	let model = fetch.save(conn).await?;
	Ok(model.try_into_model().expect("we just saved it, in SeaORM 0.12 it was just converted from a Model"))
}