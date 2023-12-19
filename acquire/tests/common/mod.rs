use acquire::strategy::{Strategy, self};
use sea_migration::{MigratorTrait, Migrator};
use sea_orm::{DatabaseConnection, error::DbErr, Set, ActiveModelTrait, ActiveModelBehavior};
use entities::{prelude::feed, entities::fetch};

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
	let url = url.into();
	let mut feed = feed::ActiveModel::new();
	feed.name = Set(format!("AutoTestFeed {} {}", strat.name(), url));
	feed.url = Set(url);
	feed.strategy = Set(strat.name().to_owned());
	
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