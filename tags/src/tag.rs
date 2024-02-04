#[derive(Debug, Clone, Copy, PartialEq,Eq)]
pub enum TagTarget {
	Feed,
	Entry,
	Both,
}

pub trait Tag {
	fn name(&self) -> &'static str;
	fn target(&self) -> TagTarget;
}