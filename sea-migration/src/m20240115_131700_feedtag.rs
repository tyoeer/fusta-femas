use sea_orm_migration::prelude::*;

use crate::m20231219_000002_add_feeds::Iden as FeedIden;
use crate::m20240113_220905_tags::Iden as TagIden;

use super::utils::*;

#[derive(Iden)]
pub enum Iden {
	FeedTag,
	FeedId,
	TagId,
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
				.table(Iden::FeedTag)
				.col(ColumnDef::new(Iden::FeedId).integer().not_null())
				.col(ColumnDef::new(Iden::TagId).integer().not_null())
				.foreign_key(
					ForeignKey::create()
					.from(Iden::FeedTag, Iden::FeedId)
					.to(FeedIden::Feed, UtilIdent::Id)
				)
				.foreign_key(
					ForeignKey::create()
					.from(Iden::FeedTag, Iden::TagId)
					.to(TagIden::Tag, UtilIdent::Id)
				)
		)
		.await
	}

	async fn down(&self, manager: &SchemaManager) -> DbRes {
		remove_table(manager, Iden::FeedTag).await
	}
}
