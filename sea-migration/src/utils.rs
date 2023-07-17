use sea_orm_migration::prelude::*;

pub type DbRes = Result<(), DbErr>;

#[derive(Iden)]
enum Ident {
	Id,
	CreatedAt,
	UpdatedAt,
}

pub async fn sql(man: &SchemaManager<'_>, sql: impl AsRef<str>) -> DbRes {
	man.get_connection()
		.execute_unprepared(sql.as_ref())
		.await
		.map(|_| ())
}

pub fn get_table_name(tcs: &TableCreateStatement) -> Result<String, DbErr> {
	let table_name_ref = tcs
		.get_table_name()
		.ok_or(DbErr::Custom("Table has no name".into()))?;
	let name = match table_name_ref {
		TableRef::Table(iden) => iden.to_string(),
		_ => {
			return Err(DbErr::Custom(
				"Can't create a table with a weird name type".into(),
			))
		}
	};

	//SQL injection go brr
	assert!(name.chars().all(char::is_alphabetic));

	Ok(name)
}

pub async fn add_table(man: &SchemaManager<'_>, tcs: &mut TableCreateStatement) -> DbRes {
	tcs.col(
		ColumnDef::new(Ident::Id)
			.integer()
			.not_null()
			.primary_key()
			.auto_increment(),
	)
	.col(
		ColumnDef::new(Ident::CreatedAt)
			.timestamp()
			.not_null()
			.default(Expr::current_timestamp()),
	)
	.col(
		ColumnDef::new(Ident::UpdatedAt)
			.timestamp()
			.not_null()
			.default(Expr::current_timestamp()),
	);
	tracing::info!("{}", tcs.to_string(SqliteQueryBuilder));

	man.create_table(tcs.to_owned()).await?;

	// Trigger for updated_at

	let table = get_table_name(tcs)?;
	sql(
		man,
		format!(
			r"CREATE TRIGGER {table}_updated_at
			AFTER UPDATE ON {table} FOR EACH ROW BEGIN
			UPDATE {table} SET updated_at = CURRENT_TIMESTAMP WHERE ROWID = NEW.ROWID;
			END;"
		),
	)
	.await
}

pub async fn remove_table(man: &SchemaManager<'_>, id: impl Iden + 'static) -> DbRes {
	//table drop also drops the trigger already
	man.drop_table(TableDropStatement::new().table(id).to_owned())
		.await
}
