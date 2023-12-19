use sea_orm_migration::{
	prelude::*,
	sea_orm::{EnumIter, Iterable}
};

use super::utils::*;
use super::m20231219_000002_add_feeds::Iden as Feed;

#[derive(Iden)]
pub enum Iden {
	Fetch,
	Url,
	Status,
	Content,
	Error,
	Log,
	FeedId,
	Strategy,
	
}

#[derive(Iden,EnumIter)]
enum Status {
	Success,
	FetchError,
	ParseError,
	EntryUpdateError,
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
				.table(Iden::Fetch)
				.col(ColumnDef::new(Iden::Url).string().not_null())
				.col(ColumnDef::new(Iden::Status).enumeration(Iden::Status, Status::iter()).not_null())
				.col(ColumnDef::new(Iden::Content).string().null())
				.col(ColumnDef::new(Iden::Error).string().null())
				.col(ColumnDef::new(Iden::Log).string().not_null().default(""))
				.col(ColumnDef::new(Iden::Strategy).string().not_null())
				.col(ColumnDef::new(Iden::FeedId).integer().not_null())
				.foreign_key(
					ForeignKey::create()
					.from(Iden::Fetch, Iden::FeedId)
					.to(Feed::Feed, UtilIdent::Id)
				)
		)
		.await
	}

	async fn down(&self, manager: &SchemaManager) -> DbRes {
		remove_table(manager, Iden::Fetch).await
	}
}
