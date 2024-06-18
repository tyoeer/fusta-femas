/*!
The part we also want to be able to deal with on the client


*/

use serde::{Serialize, Deserialize};
use entities::prelude::*;



#[derive(Debug, Clone, PartialEq,Eq, Serialize,Deserialize)]
pub enum ArgumentData {
	Bool(bool),
	Tag(tag::Ref),
}

#[derive(Debug, Clone,Copy, PartialEq,Eq, Serialize,Deserialize)]
pub enum ArgumentType {
	Bool,
	Tag,
}

impl From<ArgumentData> for ArgumentType {
	fn from(data: ArgumentData) -> Self {
		use ArgumentData as D;
		use ArgumentType as T;
		match data {
			D::Bool(_) => T::Bool,
			D::Tag(_) => T::Tag,
		}
	}
}