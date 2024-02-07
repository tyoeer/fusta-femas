use crate::ObjRef;

///Represents an object which is stored in a row in the database
pub trait Object {
	fn get_id(&self) -> i32;
	fn get_object_name() -> &'static str where Self: Sized;
	
	fn get_ref(&self) -> ObjRef<Self> where Self: Sized {
		self.get_id().into()
	}
}


/**
Something that can be (de)serialised as a trait object.
Has a generic implementation for everything that can already be (de)serialised normally.
*/
pub trait DynSer {
	//Deserialized object replaces self for trait object safety reasons
	fn deserialize_replace(&mut self, deserializer: &mut dyn erased_serde::Deserializer) -> Result<(), erased_serde::Error>;
	fn serialize(&self, serializer: &mut dyn erased_serde::Serializer) -> Result<(), erased_serde::Error>;
}

impl<T: serde::Serialize + serde::de::DeserializeOwned> DynSer for T {
	fn deserialize_replace(&mut self, deserializer: &mut dyn erased_serde::Deserializer) -> Result<(), erased_serde::Error> {
		let new = erased_serde::deserialize::<Self>(deserializer)?;
		let _ = std::mem::replace(self, new);
		Ok(())
	}
	fn serialize(&self, serializer: &mut dyn erased_serde::Serializer) -> Result<(), erased_serde::Error> {
		<Self as erased_serde::Serialize>::erased_serialize(self, serializer)
	}
	
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