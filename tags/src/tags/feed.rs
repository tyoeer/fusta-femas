use std::sync::Arc;
use entities::prelude::feed;
use sea_orm::Select;

use crate::tag::{
	Tag,
	TagTarget,
	FeedTag,
};

pub const NAME: &str = "feed";

#[derive(Debug, Clone, Default)]
pub struct Feed {}

impl FeedTag for Feed {
	fn filter_query(&self, query: Select<feed::Entity>) -> Select<feed::Entity> {
		query
		//TODO
	}
}


impl Tag for Feed {
	fn name(&self) -> &'static str {
		NAME
	}
	
	fn target(&self) -> TagTarget {
		TagTarget::Feed
	}
	fn as_feed_tag(self: Arc<Self>) -> Option<Arc<dyn FeedTag + Send + Sync>> {
		Some(self)
	}
}
