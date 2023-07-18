pub use sea_orm_migration::prelude::*;

mod utils;

mod m20230715_000001_enable_wal;
mod m20230715_000002_add_feeds;
mod m20230718_164002_add_fetches;
mod m20230718_164013_add_entries;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
	fn migrations() -> Vec<Box<dyn MigrationTrait>> {
		vec![
			Box::new(m20230715_000001_enable_wal::Migration),
			Box::new(m20230715_000002_add_feeds::Migration),
			Box::new(m20230718_164002_add_fetches::Migration),
			Box::new(m20230718_164013_add_entries::Migration),
		]
	}
}
