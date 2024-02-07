use sea_orm_migration::prelude::*;

use crate::m20240113_220905_tags::Iden as TagIden;

use super::utils::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> DbRes {
		//SQLite doesn't support multiple alterations in one statement, and SeaORM is rather transparent here
		let mut tas = Table::alter();
		tas
			.table(TagIden::Tag)
			.drop_column(TagIden::Type);
		manager.alter_table(tas).await?;
		let mut tas = Table::alter();
		tas
			.table(TagIden::Tag)
			.drop_column(TagIden::Config);
		manager.alter_table(tas).await
	}

	async fn down(&self, manager: &SchemaManager) -> DbRes {
		let mut tas = Table::alter();
		tas
			.table(TagIden::Tag)
			.add_column(ColumnDef::new(TagIden::Type).string().not_null().default("feed"));
		manager.alter_table(tas).await?;
		let mut tas = Table::alter();
		tas
			.table(TagIden::Tag)
			.add_column(ColumnDef::new(TagIden::Config).blob(BlobSize::Tiny).null());
		manager.alter_table(tas).await
	}
}
