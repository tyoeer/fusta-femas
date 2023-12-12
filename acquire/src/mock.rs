use sea_orm::*;
use entities::*;
use crate::strategy::{
	Strategy,
	EntryInfo
};

#[derive(Default, Debug, Clone)]
pub struct MockStrat;

#[async_trait::async_trait]
impl Strategy for MockStrat{
	fn name(&self) -> &'static str {
		"Mock test"
	}
	async fn fetch(&self, _conn: &DatabaseConnection, feed: &feed::Model) -> anyhow::Result<String> {
		let mock_fetched = match feed.url.as_str() {
			"ok" => "Mock ok",
			"parse error" => "Mock don't parse this",
			"fetch error" => anyhow::bail!("Mock fetch error"),
			_ => anyhow::bail!("Unknown url, don't know which mocked behaviour to use"),
		};
		Ok(mock_fetched.to_owned())
	}
	async fn parse(&self, data: &str) -> anyhow::Result<Vec<EntryInfo>> {
		match data {
			"parse error" => anyhow::bail!("This mock shouldn't be parsed"),
			_ => anyhow::bail!("idk what even is this"),
		}
	}
}