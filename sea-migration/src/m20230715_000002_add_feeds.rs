use sea_orm_migration::prelude::*;

use super::utils::*;

#[derive(Iden)]
pub enum Iden {
	Feed,
	Url,
	Name,
	Strategy,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> DbRes {
		// Replace the sample below with your own migration scripts
		add_table(
			manager,
			Table::create()
				.table(Iden::Feed)
				.col(ColumnDef::new(Iden::Url).string().not_null().unique_key())
				.col(ColumnDef::new(Iden::Name).string().not_null().unique_key())
				.col(ColumnDef::new(Iden::Strategy).string().not_null().unique_key()),
		)
		.await
	}

	async fn down(&self, manager: &SchemaManager) -> DbRes {
		remove_table(manager, Iden::Feed).await
	}
}
