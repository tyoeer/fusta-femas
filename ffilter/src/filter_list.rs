use std::sync::Arc;
use crate::filter::{Filter, Builder, GetBuilder};



#[derive(thiserror::Error,Debug)]
#[error("Could not find filter \"{0}\"")]
pub struct NotFoundError(String);


#[derive(Default, Clone)]
pub struct FilterList {
	list: Vec<Arc<dyn Filter + Send + Sync>>,
	builder_list: Vec<Builder>,
}

impl FilterList {
	pub fn new() -> Self {
		Self::default()
	}
	
	pub fn add(&mut self, filter: impl Filter + Send + Sync + 'static) {
		self.list.push(Arc::new(filter));
	}
	pub fn add_from_container(&mut self, filter: impl Into<Arc<dyn Filter + Send + Sync>>) {
		self.list.push(filter.into());
	}
	
	pub fn add_builder<Buildable: GetBuilder>(&mut self) {
		self.builder_list.push(Buildable::get_builder());
	}
	
	pub fn get_builder_by_name(&self, name: &str) -> Result<&Builder, NotFoundError> {
		self.builder_list.iter()
			.find(|f| f.get_name()==name)
			.ok_or_else(|| NotFoundError(name.to_owned()))
	}
	
	pub fn get_by_name(&self, name: &str) -> Result<&Arc<dyn Filter + Send + Sync>, NotFoundError> {
		self.list.iter()
			.find(|f| f.get_name()==name)
			.ok_or_else(|| NotFoundError(name.to_owned()))
	}
	
	pub fn iter_filters(&self) -> impl Iterator<Item = &(dyn Filter + Send + Sync)> {
		self.list.iter().map(|s| s.as_ref())
	}
}

impl std::fmt::Debug for FilterList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let names = self.iter_filters().map(|f| f.get_name()).collect::<Vec<_>>();
		f.debug_struct("FilterList").field("names", &names).finish()
	}
}