use entities::prelude::*;
use ff_object::describe::{Describe, Described};

use crate::filter::Filter;



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentData {
	Bool(bool),
	Tag(tag::Ref),
}


pub type Argument = Described<ArgumentData>;
pub type FilterData = Described<Vec<Argument>>;

pub type BuildFilterFn = fn(Vec<ArgumentData>) -> Result<Box<dyn Filter>, ArgumentError>;
pub type Builder = Described<BuildFilterFn>;

///Create Self from a [`Vec`]`<`[`ArgumentData`]`>`
pub trait Build: Sized {
	fn build(args: Vec<ArgumentData>) -> Result<Self, ArgumentError>;
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
		Described::new_with_describer::<Self>(box_dyn_filter::<Self>)
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