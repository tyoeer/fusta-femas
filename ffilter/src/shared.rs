/*!
The part we also want to be able to deal with on the client


*/


use entities::prelude::*;



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentData {
	Bool(bool),
	Tag(tag::Ref),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentType {
	Bool,
	Tag,
}