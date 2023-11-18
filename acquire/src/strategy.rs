use entities::*;
use sea_orm::*;

pub struct EntryInfo {
	feed_entry_id: String,
	title: String,
	view_url: String,
	embed_url: Option<String>,
	uploaded_at: Option<time::PrimitiveDateTime>,
}

impl EntryInfo {
	pub fn new(feed_entry_id: String, title: String, view_url: String) -> Self {
		Self {
			feed_entry_id,
			title,
			view_url,
			embed_url: None,
			uploaded_at: None,
		}
	}
	
	pub fn uploaded_at(&mut self, uploaded_at: time::PrimitiveDateTime) -> &mut Self {
		self.uploaded_at = Some(uploaded_at);
		self
	}
	
	pub fn embed_url(&mut self, embed_url: String) -> &mut Self {
		self.embed_url = Some(embed_url);
		self
	}
}

#[async_trait::async_trait]
pub trait Strategy: Send + Sync {
	//&self required to be able to call it in a dyn context
	fn name(&self) -> &'static str;
	async fn fetch(&self, conn: &DatabaseConnection, feed: &feed::Model) -> anyhow::Result<String>;
	async fn parse(&self, data: &str) -> anyhow::Result<Vec<EntryInfo>>;
}

fn error_to_string(err: anyhow::Error) -> String {
	tracing::error!("{err:?}");
	format!("{err:?}")
}

async fn update_entries(conn: &DatabaseConnection, feed: feed::Model, fetch_id: i32, entries: Vec<EntryInfo>) -> anyhow::Result<()> {
	let feed_entry_ids = entries.iter().map(|e| &e.feed_entry_id);
	let existing = feed.find_related(entry::Entity)
		.filter(entry::Column::FeedEntryId.is_in(feed_entry_ids))
		.all(conn)
		.await?;
	
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
				new.feed_id = Set(feed.id);
				new
			};
			
			model.latest_fetch_id = Set(Some(fetch_id));
			model.name = Set(entry.title);
			model.view_url = Set(entry.view_url);
			model.embed_url = Set(entry.embed_url);
			model.date = Set(entry.uploaded_at);
			model.save(conn).await?;
		}
		
		Ok(())
	})).await?;
	
	Ok(())
}

pub async fn run_strategy(conn: DatabaseConnection, feed: feed::Model, strat: &dyn Strategy) -> Result<fetch::Model, DbErr> {
	use ActiveValue::Set;
	let mut fetch = fetch::ActiveModel::new();
	fetch.feed_id = Set(feed.id);
	fetch.url = Set(feed.url.clone());
	fetch.strategy = Set(strat.name().to_owned());
	let fetched = strat.fetch(&conn, &feed).await;
	match fetched {
		Err(err) => {
			fetch.status = Set(fetch::Status::FetchError);
			fetch.error = Set(Some(error_to_string(err)));
		},
		Ok(data) => {
			fetch.content = Set(Some(data));
		},
	}
	if let Set(Some(data)) = &fetch.content {
		let parsed = strat.parse(data).await;
		match parsed {
			Err(err) => {
				fetch.status = Set(fetch::Status::ParseError);
				fetch.error = Set(Some(error_to_string(err)));
			},
			Ok(parsed) => {
				fetch.status = Set(fetch::Status::EntryUpdateError);
				let fetch = fetch.insert(&conn).await?;
				
				let res = update_entries(&conn, feed.clone(), fetch.id, parsed).await;
				let mut fetch = fetch.into_active_model();
				match res {
					Ok(_) => {
						fetch.status = Set(fetch::Status::Success);
					},
					Err(err) => {
						fetch.error = Set(Some(error_to_string(err)));
					}
				}
				//return to not do the additional insert
				let fetch = fetch.update(&conn).await?;
				return Ok(fetch);
			},
		}
	}
	
	fetch.insert(&conn).await
}

pub struct MockStrat;

#[async_trait::async_trait]
impl Strategy for MockStrat{
	fn name(&self) -> &'static str {
		"MOCK"
	}
	async fn fetch(&self, _conn: &DatabaseConnection, _feed: &feed::Model) -> anyhow::Result<String> {
		Ok("MOCK FETCHED".into())
	}
	async fn parse(&self, _data: &str) -> anyhow::Result<Vec<EntryInfo>> {
		anyhow::bail!("Mock fail")
	}
}