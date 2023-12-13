use sea_orm::*;
use entities::*;
use crate::strategy::{
	Strategy,
	EntryInfo
};

#[derive(Default, Debug, Clone)]
pub struct MockStrat {
	///Private field to prevent this from being created without a function call
	_unused: (),
}

#[async_trait::async_trait]
impl Strategy for MockStrat{
	fn name(&self) -> &'static str {
		"Mock test"
	}
	async fn fetch(&self, _conn: &DatabaseConnection, feed: &feed::Model) -> anyhow::Result<String> {
		let mock_fetched = match feed.url.as_str() {
			"ok" => "Mock ok",
			"log ok" => {
				tracing::info!("Mock fetch log");
				"Mock logged"
			},
			"log parse err" => {
				tracing::info!("Mock fetch log");
				"Mock parse log error"
			},
			"log fetch err" => {
				tracing::error!("Mock fetch err");
				anyhow::bail!("Mock fetch log error")
			},
			"parse error" => "Mock don't parse this",
			"fetch error" => anyhow::bail!("Mock fetch error"),
			_ => anyhow::bail!("Unknown url, don't know which mocked behaviour to use"),
		};
		Ok(mock_fetched.to_owned())
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
			_ => anyhow::bail!("idk what even is this"),
		};
		
		Ok(entries)
	}
}