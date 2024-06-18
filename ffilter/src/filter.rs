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

pub use crate::shared::*;



pub type Argument = Described<ArgumentData>;
pub type FilterData = Described<Vec<Argument>>;


pub type BuildFilterFn = fn(Vec<ArgumentData>) -> Result<Box<dyn Filter>, ArgumentError>;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterInfo {
	description: Described<()>,
	build_fn: BuildFilterFn,
	args_description: Vec<Described<ArgumentType>>
}

impl FilterInfo {
	pub fn new<FilterType: Build + Filter + Describe + 'static>() -> Self {
		Self {
			description: Described::new_with_describer::<FilterType>(()),
			build_fn: box_dyn_filter::<FilterType>,
			args_description: FilterType::describe_args()
		}
	}
	
	pub fn build(&self, args: Vec<ArgumentData>) -> Result<Box<dyn Filter>, ArgumentError> {
		(self.build_fn)(args)
	}
	
	pub fn get_name(&self) -> &str{
		&self.description.name
	}
}



pub type Builder = FilterInfo;

///Create Self from a [`Vec`]`<`[`ArgumentData`]`>`
pub trait Build: Sized {
	fn build(args: Vec<ArgumentData>) -> Result<Self, ArgumentError>;
	fn describe_args() -> Vec<Described<ArgumentType>>;
}

fn box_dyn_filter<T: Build + Filter + 'static>(args: Vec<ArgumentData>) -> Result<Box<dyn Filter>, ArgumentError> {
	Ok(Box::new(T::build(args)?))
}

///Get a [`Builder`] from a type that implements that required traits
pub trait GetBuilder {
	fn get_builder() -> Builder;
}

impl<This: Build + Filter + Describe + 'static> GetBuilder for This {
	fn get_builder() -> Builder {
		FilterInfo::new::<This>()
	}
}




#[derive(Debug, thiserror::Error)]
pub enum ArgumentError {
	#[error("Wrong amount of arguments: expected {expected}, found {found}")]
	WrongCount {
		expected: usize,
		found: usize,
	},
	#[error("Wrong argument type at index {index}: expected {expected:?}, found {found:?}")]
	WrongType {
		index: usize,
		expected: ArgumentData,
		found: ArgumentData,
	}
}


pub trait ReprArgument {
	fn into_arguments(self) -> Vec<Argument>;
	fn replace_from_args(&mut self, args: Vec<Argument>) -> Result<(), ArgumentError>;
}


/**
ReprArguments for trait objects.
Trait objects can't be moved due to not having a static size, which is required to call [`ReprArgument::into_arguments`] with it's self argument
*/
pub trait DynReprArgument {
	fn into_arguments(self: Box<Self>) -> Vec<Argument>;
	fn replace_from_args(&mut self, args: Vec<Argument>) -> Result<(), ArgumentError>;
}

impl<T: ReprArgument> DynReprArgument for T {
	fn into_arguments(self: Box<Self>) -> Vec<Argument> {
		<Self as ReprArgument>::into_arguments(*self)
	}
	
	fn replace_from_args(&mut self, args: Vec<Argument>) -> Result<(), ArgumentError> {
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