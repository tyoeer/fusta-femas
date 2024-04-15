/*
# Transport possibilities to client
FilterList
-> iterate
-> clone
-> into FilterData
-> collect Vec<FilterData>

# Edit on client
1. Select filter from list
	Select box with names
2. Edit arguments
	function to generate UI for Argument
	Live in `app` so `ffilter` can stay server only
3. Finish
	search button

# transport to server
-> Serialize FilterData
-> Deserialize
-> get dyn QUeryFilter from FilterList using name
-> clone
-> replace_from_args
-> dyn QueryFilter

# server filters
use dyn QueryFilter

# types exposed to client

Described
ArgumentData


# DX
impl DynFilter
	::get_name() -> verbose functionality
	::into_arguments() -> boilerplate
	::replace_from_args() -> boilerplate
impl QueryFilter
	::filter() -> functionality
*/


use sea_orm::Select;
use entities::prelude::*;
use ff_object::traits::DynSer;
use ff_object::describe::*;


pub enum ArgumentData {
	Bool(bool),
	Tag(tag::Ref),
}


pub type Argument = Described<ArgumentData>;
pub type FilterData = Described<Vec<Argument>>;


pub trait ReprArgument {
	fn into_arguments(self) -> Vec<Argument>;
	fn replace_from_args(&mut self, args: Vec<Argument>);
}

/**
ReprArguments for trait objects.
Trait objects can't be moved due to not having a static size, which is required to call [`ReprArgument::into_arguments`] with it's self argument
*/
pub trait DynReprArgument {
	fn into_arguments(self: Box<Self>) -> Vec<Argument>;
	fn replace_from_args(&mut self, args: Vec<Argument>);
}

impl<T: ReprArgument> DynReprArgument for T {
	fn into_arguments(self: Box<Self>) -> Vec<Argument> {
		<Self as ReprArgument>::into_arguments(*self)
	}
	
	fn replace_from_args(&mut self, args: Vec<Argument>) {
		<Self as ReprArgument>::replace_from_args(self, args)
	}
}

pub trait DynFilterClone {
	fn box_clone(&self) -> Box<dyn Filter>;
}

impl<T: Clone + Filter + 'static> DynFilterClone for T {
	fn box_clone(&self) -> Box<dyn Filter> {
		Box::new(self.clone())
	}
}

pub trait Filter: DynSer + DynDescribe + DynFilterClone + DynReprArgument {
	fn filter(&self, query: Select<feed::Entity>) -> Select<feed::Entity>;
}


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn filter_dyn() {
		let a = crate::filters::Fetched::default();
		let b: &dyn Filter = &a;
	}
}