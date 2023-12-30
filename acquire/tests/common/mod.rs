//Not all tests use all the stuff in here, so they generate false warnings
#![allow(dead_code)]

use acquire::{strategy::{Strategy, self}, StrategyList, mock::{FetchCommand, CommandStrat, MockStrat}};
use sea_migration::{MigratorTrait, Migrator};
use sea_orm::{DatabaseConnection, error::DbErr, Set, ActiveModelTrait, ActiveModelBehavior};
use entities::{prelude::feed, entities::fetch};
use tokio::sync::broadcast;

pub async fn db() -> Result<DatabaseConnection, DbErr> {
	let conn = sea_orm::Database::connect("sqlite::memory:").await?;
	Migrator::up(&conn, None).await?;
	
	Ok(conn)
}

pub fn init_tracing() {
	//This will fail since one test binary contains multiple tests
	let _ = tracing_subscriber::fmt()
		.with_max_level(::tracing::Level::DEBUG)
		.with_test_writer()
		.try_init();
}

pub async fn init() -> Result<DatabaseConnection, DbErr> {
	init_tracing();
	db().await
}

pub async fn feed(
	url: impl Into<String>,
	strat: &dyn acquire::strategy::Strategy,
	db: &DatabaseConnection
) -> Result<feed::Model, DbErr> {
	feed_strat_name(url, strat.name(), db).await
}

pub async fn feed_strat_name(
	url: impl Into<String>,
	strat_name: impl AsRef<str>,
	db: &DatabaseConnection
) -> Result<feed::Model, DbErr> {
	let url = url.into();
	let strat_name = strat_name.as_ref();
	let mut feed = feed::ActiveModel::new();
	feed.name = Set(format!("AutoTestFeed {} {}", strat_name, url));
	feed.url = Set(url);
	feed.strategy = Set(strat_name.to_owned());
	
	let feed = feed.insert(db).await?;
	
	Ok(feed)
}

pub async fn run_strategy(db: &DatabaseConnection, feed: &feed::Model, strategy: &dyn Strategy) -> Result<fetch::Model, DbErr> {
	let fetch = strategy::run_strategy(db, feed, strategy).await?;
	
	if let Some(ref error) = fetch.error {
		//Formatted like this to preserve newlines
		tracing::error!("Fetch error:\n{}", error);
	}
	//Formatted like this to preserve newlines
	tracing::debug!("Fetch log:\n{}", fetch.log);
	
	Ok(fetch)
}

pub fn single_strat_list(strat: impl Strategy + Send + Sync + 'static) -> StrategyList {
	let mut list = StrategyList::new();
	list.add(strat);
	list
}

pub fn cmd_strats() -> (broadcast::Sender<FetchCommand>, StrategyList) {
	let mut list = StrategyList::new();
	
	let cmd_strat = CommandStrat::new();
	let sender = cmd_strat.sender();
	
	list.add(cmd_strat);
	list.add(MockStrat::default());
	
	(sender, list)
}