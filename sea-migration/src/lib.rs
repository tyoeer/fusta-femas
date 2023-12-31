pub use sea_orm_migration::prelude::*;

mod utils;

mod m20231219_000001_enable_wal;
mod m20231219_000002_add_feeds;
mod m20231219_000003_add_fetches;
mod m20231219_000004_add_entries;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
	fn migrations() -> Vec<Box<dyn MigrationTrait>> {
		vec![
			Box::new(m20231219_000001_enable_wal::Migration),
			Box::new(m20231219_000002_add_feeds::Migration),
			Box::new(m20231219_000003_add_fetches::Migration),
			Box::new(m20231219_000004_add_entries::Migration),
		]
	}
}
