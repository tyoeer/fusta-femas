mod common;
use common::{init, feed, run_strategy};
use sea_orm::{DbErr, ModelTrait, PaginatorTrait, QueryOrder};
use acquire::{
	strategy::Strategy,
	mock::MockStrat
};
use entities::prelude::*;

///A simple test that can copy/pasted to be the basis of other tests
#[tokio::test]
async fn basic() -> Result<(), DbErr> {
	let db = init().await?;
	let strat = MockStrat::default();
	let feed = feed("ok", &strat, &db).await?;
	
	let fetch = run_strategy(&db, &feed, &strat).await?;
	
	assert_eq!(fetch.status, fetch::Status::Success);
	
	Ok(())
}

///The created fetch looks good
#[tokio::test]
async fn fetch() -> Result<(), DbErr> {
	let db = init().await?;
	let strat = MockStrat::default();
	let feed = feed("ok", &strat, &db).await?;
	
	let fetch = run_strategy(&db, &feed, &strat).await?;
	
	assert_eq!(fetch.status, fetch::Status::Success);
	assert_eq!(fetch.feed_id, feed.id);
	assert_eq!(fetch.strategy, strat.name());
	assert!(fetch.error.is_none());
	assert!(fetch.content.is_some());
	
	Ok(())
}

///Thinks look alright when a fetch error occurred
#[tokio::test]
async fn fetch_error() -> Result<(), DbErr> {
	let db = init().await?;
	let strat = MockStrat::default();
	let feed = feed("fetch error", &strat, &db).await?;
	
	let fetch = run_strategy(&db, &feed, &strat).await?;
	
	assert_eq!(fetch.status, fetch::Status::FetchError);
	assert!(fetch.error.is_some());
	assert!(fetch.content.is_none());
	
	Ok(())
}

///Thinks look alright when a parse error occurred
#[tokio::test]
async fn parse_error() -> Result<(), DbErr> {
	let db = init().await?;
	let strat = MockStrat::default();
	let feed = feed("parse error", &strat, &db).await?;
	
	let fetch = run_strategy(&db, &feed, &strat).await?;
	
	assert_eq!(fetch.status, fetch::Status::ParseError);
	assert!(fetch.error.is_some());
	assert!(fetch.content.is_some());
	
	Ok(())
}

///tracing logs get collected
#[tokio::test]
async fn logs() -> Result<(), DbErr> {
	let db = init().await?;
	let strat = MockStrat::default();
	
	let feed_ok = feed("log ok", &strat, &db).await?;
	let feed_fetch_err = feed("log fetch err", &strat, &db).await?;
	let feed_parse_err = feed("log parse err", &strat, &db).await?;
	
	let fetch = run_strategy(&db, &feed_ok, &strat).await?;
	
	assert_eq!(fetch.status, fetch::Status::Success);
	assert!(fetch.log.contains("Mock fetch log"));
	assert!(fetch.log.contains("Mock parse log"));
	
	let fetch = run_strategy(&db, &feed_fetch_err, &strat).await?;
	
	assert_eq!(fetch.status, fetch::Status::FetchError);
	assert!(fetch.log.contains("Mock fetch err"));
	
	let fetch = run_strategy(&db, &feed_parse_err, &strat).await?;
	
	assert_eq!(fetch.status, fetch::Status::ParseError);
	assert!(fetch.log.contains("Mock fetch log"));
	assert!(fetch.log.contains("Mock parse err"));
	
	Ok(())
}

//entries get saved + fetches get tracked
#[tokio::test]
async fn entries() -> Result<(), DbErr> {
	let db = init().await?;
	let strat = MockStrat::default();
	let feed = feed("10n5", &strat, &db).await?;
	
	let fetch1 = run_strategy(&db, &feed, &strat).await?;
	
	assert_eq!(fetch1.status, fetch::Status::Success);
	assert_eq!(10, fetch1.find_related(entry::Entity).count(&db).await? );
	
	let entry_0 = fetch1.find_related(entry::Entity).order_by_asc(entry::Column::ProducedDate).one(&db).await?.expect("just created");
	let entry_9 = fetch1.find_related(entry::Entity).order_by_desc(entry::Column::ProducedDate).one(&db).await?.expect("just created");
	
	assert_eq!(1, entry_0.find_related(fetch::Entity).count(&db).await? );
	assert_eq!(1, entry_9.find_related(fetch::Entity).count(&db).await? );
	
	let fetch2 = run_strategy(&db, &feed, &strat).await?;
	
	assert_eq!(fetch2.status, fetch::Status::Success);
	assert_eq!(10, fetch2.find_related(entry::Entity).count(&db).await? );
	assert_eq!(10, fetch1.find_related(entry::Entity).count(&db).await? );
	
	let entry_5 = fetch2.find_related(entry::Entity).order_by_asc(entry::Column::ProducedDate).one(&db).await?.expect("just created");
	let entry_14 = fetch2.find_related(entry::Entity).order_by_desc(entry::Column::ProducedDate).one(&db).await?.expect("just created");
	
	assert_eq!(1, entry_0.find_related(fetch::Entity).count(&db).await? );
	assert_eq!(2, entry_5.find_related(fetch::Entity).count(&db).await? );
	assert_eq!(2, entry_9.find_related(fetch::Entity).count(&db).await? );
	assert_eq!(1, entry_14.find_related(fetch::Entity).count(&db).await? );
	
	Ok(())
}