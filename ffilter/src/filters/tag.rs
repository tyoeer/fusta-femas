use entities::prelude::*;
use ff_object::describe::{Describe, Described};
use sea_orm::prelude::Select;
use serde::{Deserialize, Serialize};

use crate::filter::{
	Argument, ArgumentData, Filter, ReprArgument
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

	fn replace_from_args(&mut self, args: Vec<Argument>) {
		//Can't destructure something without a const size for some reason
		let bs: [Argument; 1] = args.try_into().expect("there should only be 1 argument");
		let [
			Described {
				data: ArgumentData::Tag(tag),
				..
			}
		] = bs else {
			panic!("Wrong argument type(s)");
		};

		self.tag = tag;
	}
}