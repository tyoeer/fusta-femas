use std::sync::Arc;
use entities::prelude::*;
use sea_orm::Select;

use crate::tag::{
	Tag,
	TagTarget,
	FeedTag,
};

pub const NAME: &str = "feed";

#[derive(Debug, Clone, Default)]
pub struct FeedManual {}

impl FeedTag for FeedManual {
	fn filter_query(&self, tag: tag::Model, query: Select<feed::Entity>) -> Select<feed::Entity> {
		query
		//TODO
	}
}


impl Tag for FeedManual {
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