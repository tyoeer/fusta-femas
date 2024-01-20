use std::marker::PhantomData;
use serde::{Serialize, Deserialize};

use super::Object;

#[cfg(feature="orm")]
use sea_orm::{
	*,
	sea_query::*,
};

///A reference to a _possible_ object in the database.
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObjRef<Model, Id: Clone = i32> {
	id: Id,
	#[serde(skip)]
	_ignore: PhantomData<Model>
}

impl<Model, Id: Clone> Clone for ObjRef<Model, Id> {
	fn clone(&self) -> Self {
		Self::new(self.id())
	}
}

impl<Model, Id: Clone + PartialEq> PartialEq for ObjRef<Model, Id> {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl<Model, Id: Clone + Eq> Eq for ObjRef<Model, Id> {}

impl<Model, Id: Clone> ObjRef<Model, Id> {
	pub fn new(id: Id) -> Self {
		Self {
			id,
			_ignore: PhantomData,
		}
	}
	
	///Returns a clone of the id
	pub fn id(&self) -> Id {
		self.id.clone()
	}
	
	pub fn into_id(self) -> Id {
		self.id
	}
	
	pub fn id_ref(&self) -> &Id {
		&self.id
	}
}

impl<Model: Object> From<Model> for ObjRef<Model> {
	fn from(model: Model) -> Self {
		Self::new(model.get_id())
	}
}

#[cfg(feature="orm")]
type IdType<Model> = <<<Model as ModelTrait>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType;
#[cfg(feature="orm")]
type Entity<Model> = <Model as ModelTrait>::Entity;

#[cfg(feature="orm")]
impl<Model: ModelTrait> ObjRef<
	Model,
	IdType<Model>,
> where
	<<<Model as ModelTrait>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: Clone,
{
	/**
	Filters the given query to only select the rows relating to the object we're referencing.
	Does not make sure the query has the table we're checking, so this can lead to an error.
	*/
	pub fn filter_unchecked<SelectBaseEntity: EntityTrait>(&self, mut query: Select<SelectBaseEntity>) -> Select<SelectBaseEntity> {
		//Code copied from https://docs.rs/sea-orm/latest/src/sea_orm/entity/base_entity.rs.html#268-286
		//Not quite sure where this strum::IntoEnumIterator trait impl comes from
		let mut keys = <Entity<Model> as EntityTrait>::PrimaryKey::iter();
		for v in self.id().into_value_tuple() {
			if let Some(key) = keys.next() {
				let col = key.into_column();
				query = query.filter(col.eq(v));
			} else {
				panic!("primary key arity mismatch");
			}
		}
		if keys.next().is_some() {
			panic!("primary key arity mismatch");
		}
		query
	}
	
	///Filters to the query to row related to the referenced object.
	pub fn filter_related<SelectBaseEntity: Related<Entity<Model>> + EntityTrait>(&self, query: Select<SelectBaseEntity>) -> Select<SelectBaseEntity> {
		let query = query.inner_join(Entity::<Model>::default());
		self.filter_unchecked(query)
	}
	
	pub fn query_entity() -> Select<Entity<Model>> {
		<Entity::<Model> as EntityTrait>::find()
	}
}