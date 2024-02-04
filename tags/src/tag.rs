use sea_orm::Select;
use std::sync::Arc;
use entities::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq,Eq)]
pub enum TagTarget {
	Feed,
	Entry,
	Both,
}

pub trait Tag: Send + Sync {
	fn name(&self) -> &'static str;
	fn target(&self) -> TagTarget;
	fn as_feed_tag(self: Arc<Self>) -> Option<Arc<dyn FeedTag + Send + Sync>> {
		None
	}
	fn as_entry_tag(self: Arc<Self>) -> Option<Arc<dyn EntryTag + Send + Sync>> {
		None
	}
}

pub trait FeedTag: Tag {
	fn filter_query(&self, query: Select<feed::Entity>) -> Select<feed::Entity>;
}
pub trait EntryTag: Tag {
	fn filter_query(&self, query: Select<entry::Entity>) -> Select<entry::Entity>;
}