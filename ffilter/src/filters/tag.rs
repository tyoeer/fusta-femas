use entities::prelude::*;
use ff_object::describe::{Describe, Described};
use sea_orm::prelude::Select;
use serde::{Deserialize, Serialize};

use crate::filter::{
	Argument, ArgumentData, ArgumentError, Build, Filter, ReprArgument
};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
	tag: tag::Ref,
}

//Needed for easily adding it in setup
impl Default for Tag {
	fn default() -> Self {
		Self {
			tag: tag::Ref::new(-1),
		}
	}
}

impl Filter for Tag {
	fn filter(&self, query: Select<feed::Entity>) -> Select<feed::Entity> {
		self.tag.filter_related(query)
	}
}

impl Describe for Tag {
	const NAME: &'static str = "tag";
}

impl Build for Tag {
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

		let tag = match first_arg {
			ArgumentData::Tag(tag) => tag,
			other => return Err(
				ArgumentError::WrongType {
					index: 0,
					expected: ArgumentData::Tag(tag::Ref::new(-1)),
					found: other
				}
			),
		};

		Ok(Self {
			tag
		})
	}
}

impl ReprArgument for Tag {
	fn into_arguments(self) -> Vec<Argument> {
		vec![
			Described::custom_new(
				ArgumentData::Tag(self.tag),
				"tag".to_owned(),
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
		
		let tag = match first_arg {
			ArgumentData::Tag(tag) => tag,
			other => return Err(
				ArgumentError::WrongType {
					index: 0,
					expected: ArgumentData::Tag(self.tag.clone()),
					found: other
				}
			),
		};
		
		self.tag = tag;
		
		Ok(())
	}
}