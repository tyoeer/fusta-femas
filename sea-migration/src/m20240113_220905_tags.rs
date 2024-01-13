use sea_orm_migration::prelude::*;

use super::utils::*;

#[derive(Iden)]
pub enum Iden {
	Tag,
	Title,
	Type,
	Config,
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
				.table(Iden::Tag)
				.col(ColumnDef::new(Iden::Title).string().not_null())
				.col(ColumnDef::new(Iden::Type).string().not_null())
				.col(ColumnDef::new(Iden::Config).blob(BlobSize::Tiny).null()),
		)
		.await
	}

	async fn down(&self, manager: &SchemaManager) -> DbRes {
		remove_table(manager, Iden::Tag).await
	}
}
