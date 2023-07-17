pub use sea_orm_migration::prelude::*;

mod utils;

mod m20230715_000001_enable_wal;
mod m20230715_000002_add_feeds;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
	fn migrations() -> Vec<Box<dyn MigrationTrait>> {
		vec![
			Box::new(m20230715_000001_enable_wal::Migration),
			Box::new(m20230715_000002_add_feeds::Migration),
		]
	}
}
