use sea_orm_migration::prelude::*;

use super::utils::*;
use super::m20230715_000002_add_feeds::Iden as Feed;
use super::m20230718_164002_add_fetches::Iden as Fetch;

#[derive(Iden)]
pub enum Iden {
	Entry,
	FeedId,
	LatestFetchId,
	FeedEntryId,
	ViewUrl,
	Viewed,
	EmbedUrl,
	Name,
	ProducedDate,
	ProducedTime,
}

#[derive(Iden)]
pub enum FEIden {
	FetchEntry,
	FetchId,
	EntryId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> DbRes {
		add_table(
			manager,
			Table::create()
				.table(Iden::Entry)
				.col(ColumnDef::new(Iden::Name).string().not_null())
				.col(ColumnDef::new(Iden::ViewUrl).string().not_null())
				.col(ColumnDef::new(Iden::EmbedUrl).string().null())
				.col(ColumnDef::new(Iden::Viewed).boolean().not_null().default(false))
				.col(ColumnDef::new(Iden::FeedEntryId).string().not_null())
				.col(ColumnDef::new(Iden::FeedId).integer().not_null())
				.col(ColumnDef::new(Iden::LatestFetchId).integer().null())
				.col(ColumnDef::new(Iden::ProducedDate).date().not_null())
				.col(ColumnDef::new(Iden::ProducedTime).time().null())
				.foreign_key(
					ForeignKey::create()
					.from(Iden::Entry, Iden::FeedId)
					.to(Feed::Feed, UtilIdent::Id)
				)
				.foreign_key(
					ForeignKey::create()
					.from(Iden::Entry, Iden::LatestFetchId)
					.to(Fetch::Fetch, UtilIdent::Id)
				)
		)
		.await?;
		
		add_table(
			manager,
			Table::create()
				.table(FEIden::FetchEntry)
				.col(ColumnDef::new(FEIden::FetchId).integer().not_null())
				.col(ColumnDef::new(FEIden::EntryId).integer().not_null())
				.foreign_key(
					ForeignKey::create()
						.from(FEIden::FetchEntry, FEIden::FetchId)
						.to(Fetch::Fetch, UtilIdent::Id)
				)
				.foreign_key(
					ForeignKey::create()
						.from(FEIden::FetchEntry, FEIden::EntryId)
						.to(Iden::Entry, UtilIdent::Id)
				)
		).await?;
		
		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> DbRes {
		remove_table(manager, Iden::Entry).await?;
		remove_table(manager, FEIden::FetchEntry).await?;
		
		Ok(())
	}
}
