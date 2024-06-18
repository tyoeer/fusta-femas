use entities::prelude::*;
use ff_object::describe::{Describe, Described};
use sea_orm::prelude::Select;
use serde::{Deserialize, Serialize};

use crate::filter::{
	Argument, ArgumentData, ArgumentError, ArgumentType, Build, Filter, ReprArgument
};


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ArgTest {
	bool: bool,
}

impl Filter for ArgTest {
	fn filter(&self, query: Select<feed::Entity>) -> Select<feed::Entity> {
		query
	}
}

impl Describe for ArgTest {
	const NAME: &'static str = "arg_test";
}

impl Build for ArgTest {
	fn build(args: Vec<ArgumentData>) -> Result<Self, ArgumentError> {
		//Can't move-destructure something without a const size for some reason
		let sized_args = match <[ArgumentData; 1]>::try_from(args) {
			Ok(sized) => sized,
			Err(original) => return Err(
				ArgumentError::WrongCount {
					expected: 1,
					found: original.len()
				}
			),
		};

		let [
			first_arg,
		] = sized_args;

		let bool = match first_arg {
			ArgumentData::Bool(bool) => bool,
			other => return Err(
				ArgumentError::WrongType {
					index: 0,
					expected: ArgumentData::Bool(false),
					found: other
				}
			),
		};

		Ok(Self {
			bool,
		})
	}
	
	fn describe_args() -> Vec<Described<ArgumentType>> {
		vec![
			Described::custom_new(
				ArgumentType::Bool,
				"bool".to_owned(),
				None
			)
		]
	}
}

impl ReprArgument for ArgTest {
	fn into_arguments(self) -> Vec<Argument> {
		vec![
			Described::custom_new(
				ArgumentData::Bool(self.bool),
				"bool".to_owned(),
				None
			)
		]
	}

	fn replace_from_args(&mut self, args: Vec<Argument>) -> Result<(), ArgumentError> {
		//Can't move-destructure something without a const size for some reason
		let sized_args = match <[Argument; 1]>::try_from(args) {
			Ok(sized) => sized,
			Err(original) => return Err(
				ArgumentError::WrongCount {
					expected: 1,
					found: original.len()
				}
			),
		};
		
		let [
			Described {
				data: first_arg,
				..
			}
		] = sized_args;
		
		let bool = match first_arg {
			ArgumentData::Bool(bool) => bool,
			other => return Err(
				ArgumentError::WrongType {
					index: 0,
					expected: ArgumentData::Bool(self.bool.clone()),
					found: other
				}
			),
		};
		
		self.bool = bool;
		
		Ok(())
	}
}