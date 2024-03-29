use entities::prelude::*;
use ff_object::describe::Describe;
use sea_orm::{prelude::Select, QuerySelect};
use serde::{Deserialize, Serialize};

use crate::filter::{
	Filter,
	ReprArgument
};


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Fetched;

impl Filter for Fetched {
	fn filter(&self, query: Select<feed::Entity>) -> Select<feed::Entity> {
		query
			//inner join makes sure the relation exists
			.inner_join(fetch::Entity)
			//distinct prevents duplication from multiple fetches existing
			.distinct()
	}
}

impl Describe for Fetched {
	const NAME: &'static str = "fetched";
}

impl ReprArgument for Fetched {
	fn into_arguments(self) -> Vec<crate::filter::Argument> {
		Vec::new()
	}

	fn replace_from_args(&mut self, _args: Vec<crate::filter::Argument>) {
		//do nothing, since we have no arguments
	}
}