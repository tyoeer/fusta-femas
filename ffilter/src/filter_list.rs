use std::sync::Arc;
use crate::filter::Filter;


#[derive(thiserror::Error,Debug)]
#[error("Could not find filter \"{0}\"")]
pub struct NotFoundError(String);


#[derive(Default, Clone)]
pub struct FilterList {
	list: Vec<Arc<dyn Filter + Send + Sync>>
}

impl FilterList {
	pub fn new() -> Self {
		Self::default()
	}
	
	pub fn add(&mut self, strat: impl Filter + Send + Sync + 'static) {
		self.list.push(Arc::new(strat));
	}
	pub fn add_from_container(&mut self, strat: impl Into<Arc<dyn Filter + Send + Sync>>) {
		self.list.push(strat.into());
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