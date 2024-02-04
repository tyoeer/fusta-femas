use crate::tag::{
	Tag,
	TagTarget,
};

pub const NAME: &str = "feed";

#[derive(Debug, Clone, Default)]
pub struct Feed {}

impl Tag for Feed {
	fn name(&self) -> &'static str {
		NAME
	}
	
	fn target(&self) -> TagTarget {
		TagTarget::Feed
	}
}