use entities::prelude::*;
use ff_object::describe::{Describe, Described};
use sea_orm::prelude::Select;
use serde::{Deserialize, Serialize};

use crate::filter::{
	Argument, ArgumentData, Filter, ReprArgument
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

	fn replace_from_args(&mut self, args: Vec<Argument>) {
		let [
			Described {
				data: ArgumentData::Bool(bool),
				..
			}
		] = *args.into_boxed_slice() else {
			panic!("Wrong arguments");
		};
		
		self.bool = bool;
	}
}