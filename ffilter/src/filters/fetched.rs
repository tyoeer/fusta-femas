use entities::prelude::feed;
use ff_object::describe::Describe;
use sea_orm::prelude::Select;
use serde::{Deserialize, Serialize};

use crate::filter::{
	Filter,
	ReprArgument
};


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Fetched;

impl Filter for Fetched {
	fn filter(&self, query: Select<feed::Entity>) -> Select<feed::Entity> {
		todo!()
	}
}

impl Describe for Fetched {
	const NAME: &'static str = "viewed";
}

impl ReprArgument for Fetched {
	fn into_arguments(self) -> Vec<crate::filter::Argument> {
		Vec::new()
	}

	fn replace_from_args(&mut self, args: Vec<crate::filter::Argument>) {
		//do nothing, since we have no arguments
	}
}