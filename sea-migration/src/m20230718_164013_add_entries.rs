use sea_orm_migration::prelude::*;

use super::utils::*;
use super::m20230715_000002_add_feeds::Iden as Feed;
use super::m20230718_164002_add_fetches::Iden as Fetch;

#[derive(Iden)]
pub enum Iden {
	Entry,
	FeedId,
	FetchId,
	FeedEntryId,
	ViewUrl,
	Viewed,
	EmbedUrl,
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
				.table(Iden::Entry)
				.col(ColumnDef::new(Iden::ViewUrl).string().not_null())
				.col(ColumnDef::new(Iden::EmbedUrl).string().null())
				.col(ColumnDef::new(Iden::Viewed).boolean().not_null().default(false))
				.col(ColumnDef::new(Iden::FeedEntryId).string().not_null())
				.col(ColumnDef::new(Iden::FeedId).integer().not_null())
				.col(ColumnDef::new(Iden::FetchId).integer().not_null())
				.foreign_key(
					ForeignKey::create()
					.from(Iden::Entry, Iden::FeedId)
					.to(Feed::Feed, UtilIdent::Id)
				)
				.foreign_key(
					ForeignKey::create()
					.from(Iden::Entry, Iden::FetchId)
					.to(Fetch::Fetch, UtilIdent::Id)
				)
		)
		.await
	}

	async fn down(&self, manager: &SchemaManager) -> DbRes {
		remove_table(manager, Iden::Entry).await
	}
}
