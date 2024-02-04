use std::sync::Arc;
use super::tag::{
	Tag,
	FeedTag,
	EntryTag,
};

#[derive(thiserror::Error,Debug)]
#[error("Could not find tag \"{0}\"")]
pub struct NotFoundError(String);


#[derive(Default,Clone)]
pub struct TagList {
	feed_tags: Vec<Arc<dyn FeedTag + Send + Sync>>,
	entry_tags: Vec<Arc<dyn EntryTag + Send + Sync>>,
}

impl TagList {
	pub fn new() -> Self {
		Self::default()
	}
	
	pub fn add_from_container(&mut self, tag: impl Into<Arc<dyn Tag + Send + Sync>>) {
		let tag = tag.into();
		if let Some(feed_tag) = tag.clone().as_feed_tag() {
			self.feed_tags.push(feed_tag);
		}
		if let Some(entry_tag) = tag.as_entry_tag() {
			self.entry_tags.push(entry_tag);
		}
	}
	
	pub fn add_feed_tag(&mut self, tag: impl FeedTag + Send + Sync + 'static) {
		self.feed_tags.push(Arc::new(tag));
	}
	pub fn add_feed_tag_from_container(&mut self, tag: impl Into<Arc<dyn FeedTag + Send + Sync>>) {
		self.feed_tags.push(tag.into());
	}
	
	pub fn get_feed_tag_by_name(&self, name: &str) -> Result<&Arc<dyn FeedTag + Send + Sync>, NotFoundError> {
		self.feed_tags.iter()
			.find(|tag| tag.name()==name)
			.ok_or_else(|| NotFoundError(name.to_owned()))
	}
	
	pub fn iter_feed_tags(&self) -> impl Iterator<Item = &(dyn FeedTag + Send + Sync)> {
		self.feed_tags.iter().map(|tag| tag.as_ref())
	}
	
	
	pub fn add_entry_tag(&mut self, tag: impl EntryTag + Send + Sync + 'static) {
		self.entry_tags.push(Arc::new(tag));
	}
	pub fn add_entry_tag_from_container(&mut self, tag: impl Into<Arc<dyn EntryTag + Send + Sync>>) {
		self.entry_tags.push(tag.into());
	}
	
	pub fn get_entry_tag_by_name(&self, name: &str) -> Result<&Arc<dyn EntryTag + Send + Sync>, NotFoundError> {
		self.entry_tags.iter()
			.find(|tag| tag.name()==name)
			.ok_or_else(|| NotFoundError(name.to_owned()))
	}
	
	pub fn iter_entry_tags(&self) -> impl Iterator<Item = &(dyn EntryTag + Send + Sync)> {
		self.entry_tags.iter().map(|tag| tag.as_ref())
	}
}