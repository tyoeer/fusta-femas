///Represents an object which is stored in a row in the database
pub trait Object {
	fn get_id(&self) -> i32;
	fn get_object_name() -> &'static str where Self: Sized;
}


#[cfg(feature="orm")]
use sea_orm::{
	Select,
	SelectModel,
	Selector,
	QuerySelect,
	FromQueryResult,
	EntityTrait,
	ColumnTrait,
};

/**

A struct that can be queried from the database.
Provides conveniences that combine [FromQueryResult] and an [Entity](EntityTrait).

The methods you want to edit are:
- [`columns`](View::columns)
- [`order`](View::order)

*/
#[cfg(feature="orm")]
pub trait View: FromQueryResult {
	///Represents the table from which this struct gets its data
	type Entity: EntityTrait;
	
	///Specifies which columns this struct selects
	fn columns() -> impl Iterator<Item = impl ColumnTrait>;
	
	///Modifies `query` to be sorted according to a standard
	fn order(query: Select<Self::Entity>) -> Select<Self::Entity> {
		query
	}
	
	
	fn query(modifier: impl FnOnce(Select<Self::Entity>) -> Select<Self::Entity>) -> Selector<SelectModel<Self>> {
		let query = Self::Entity::find();
		let query = modifier(query);
		Self::from_query(query)
	}
	
	
	fn from_query(query: Select<Self::Entity>) -> sea_orm::Selector<SelectModel<Self>> {
		let query = Self::order(query);
		let query = Self::select_only_columns(query);
		query.into_model::<Self>()
	}
	
	fn select_only_columns(query: Select<Self::Entity>) -> Select<Self::Entity> {
		query
			.select_only()
			.columns(Self::columns())
	}
}