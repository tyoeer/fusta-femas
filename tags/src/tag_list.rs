use std::sync::Arc;
use super::tag::Tag;

#[derive(thiserror::Error,Debug)]
#[error("Could not find tag \"{0}\"")]
pub struct NotFoundError(String);


#[derive(Default,Clone)]
pub struct TagList {
	list: Vec<Arc<dyn Tag + Send + Sync>>
}

impl TagList {
	pub fn new() -> Self {
		Self::default()
	}
	
	pub fn add(&mut self, tag: impl Tag + Send + Sync + 'static) {
		self.list.push(Arc::new(tag));
	}
	pub fn add_from_container(&mut self, tag: impl Into<Arc<dyn Tag + Send + Sync>>) {
		self.list.push(tag.into());
	}
	
	pub fn get_by_name(&self, name: &str) -> Result<&Arc<dyn Tag + Send + Sync>, NotFoundError> {
		self.list.iter()
			.find(|tag| tag.name()==name)
			.ok_or_else(|| NotFoundError(name.to_owned()))
	}
	
	pub fn iter_tags(&self) -> impl Iterator<Item = &(dyn Tag + Send + Sync)> {
		self.list.iter().map(|tag| tag.as_ref())
	}
}