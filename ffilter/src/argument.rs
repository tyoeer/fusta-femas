use entities::prelude::*;
use ff_object::describe::Described;



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentData {
	Bool(bool),
	Tag(tag::Ref),
}


pub type Argument = Described<ArgumentData>;
pub type FilterData = Described<Vec<Argument>>;


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