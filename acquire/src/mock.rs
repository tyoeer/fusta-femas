use sea_orm::*;
use entities::prelude::*;
use tokio::sync::broadcast;
use crate::strategy::{
	Strategy,
	EntryInfo
};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct MockStrat {
	///Private field to prevent this from being created without a function call
	_unused: (),
}

#[async_trait::async_trait]
impl Strategy for MockStrat {
	fn name(&self) -> &'static str {
		"Mock test"
	}
	
	async fn fetch(&self, conn: &DatabaseConnection, feed: &feed::Model) -> anyhow::Result<String> {
		let mock_fetched = match feed.url.as_str() {
			"ok" => "Mock ok".to_owned(),
			"log ok" => {
				tracing::info!("Mock fetch log");
				"Mock logged".to_owned()
			},
			"log parse err" => {
				tracing::info!("Mock fetch log");
				"Mock parse log error".to_owned()
			},
			"log fetch err" => {
				tracing::error!("Mock fetch err");
				anyhow::bail!("Mock fetch log error")
			},
			"parse error" => "Mock don't parse this".to_owned(),
			"fetch error" => anyhow::bail!("Mock fetch error"),
			entries if entries.contains('n') => {
				let (n, new) = entries.split_once('n').expect("we just checked in the match guard");
				let n = str::parse::<i32>(n)?;
				let new = str::parse::<i32>(new)?;
				let maybe_last_entry = feed.find_related(entry::Entity)
					.order_by_desc(entry::Column::ProducedDate)
					.one(conn).await?;
				let last = match maybe_last_entry {
					None => 0,
					Some(entry) => str::parse(&entry.feed_entry_id)?,
				};
				let double = n - new;
				let lower = 0.max(last - double + 1);
				format!("{}-{}", lower, lower + n)
			},
			_ => anyhow::bail!("Unknown url, don't know which mocked behaviour to use"),
		};
		Ok(mock_fetched)
	}
	
	async fn parse(&self, data: &str) -> anyhow::Result<Vec<EntryInfo>> {
		let entries = match data {
			"Mock ok" => Vec::new(),
			"Mock logged" => {
				tracing::info!("Mock parse log");
				Vec::new()
			},
			"Mock parse log error" => {
				tracing::error!("Mock parse err");
				anyhow::bail!("Mock parse log error");
			},
			"parse error" => anyhow::bail!("This mock shouldn't be parsed"),
			range if range.contains('-') => {
				let (from, to) = range.split_once('-').expect("we just checked in the match guard");
				let from = str::parse::<i32>(from)?;
				let to = str::parse::<i32>(to)?;
				
				let start_date: time::Date = time::Date::from_calendar_date(2000, time::Month::January, 1)?;
				
				(from..to).map(|i| EntryInfo::new(
					i.to_string(),
					format!("Entry {i}"),
					format!("example.com/{i}"),
					start_date + time::Duration::days(i.into())
				)).collect::<Vec<_>>()	
			},
			_ => anyhow::bail!("idk what even is this"),
		};
		
		Ok(entries)
	}
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetchCommand {
	Fetch(i32),
	Parse(i32),
}

#[derive(serde::Serialize,serde::Deserialize)]
#[serde(default)]
pub struct CommandStrat {
	#[serde(skip)]
	send: broadcast::Sender<FetchCommand>,
	#[serde(skip)]
	recv: broadcast::Receiver<FetchCommand>,
}

impl CommandStrat {
	pub fn new() -> Self {
		let (send, recv) = broadcast::channel(64);
		Self {
			send,
			recv,
		}
	}
	
	pub fn sender(&self) -> broadcast::Sender<FetchCommand> {
		self.send.clone()
	}
}

impl Default for CommandStrat {
	fn default() -> Self {
		Self::new()
	}
}

#[async_trait::async_trait]
impl Strategy for CommandStrat {
	fn name(&self) -> &'static str {
		"commandable mock"
	}
	async fn fetch(&self, _conn: &DatabaseConnection, feed: &feed::Model) -> anyhow::Result<String> {
		let id = feed.id;
		
		let mut recv = self.recv.resubscribe();
		loop {
			let mes = recv.recv().await?;
			if mes==FetchCommand::Fetch(id) {
				break;
			}
		}
		
		Ok(id.to_string())
	}
	async fn parse(&self, data: &str) -> anyhow::Result<Vec<EntryInfo>> {
		let id = data.parse()?;
				
		let mut recv = self.recv.resubscribe();
		loop {
			let mes = recv.recv().await?;
			if mes==FetchCommand::Parse(id) {
				break;
			}
		}
		
		Ok(Vec::new())
	}
}