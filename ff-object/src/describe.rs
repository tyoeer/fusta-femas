use serde::{Deserialize, Serialize};

/**
Generic runtime data that is described at runtime using a name and an optional description.

Also see [`Describe`] for a compile-time description of a type.
*/
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Described<Data> {
	pub name: String,
	pub description: Option<String>,
	pub data: Data,
}

impl<Data> Described<Data> {
	pub fn custom_new(data: Data, name: String, description: Option<String>) -> Self {
		Self {
			name,
			description,
			data,
		}
	}
	
	///Describe data using the description of a separate type implementing [`Describe`]
	pub fn new_with_describer<Describer: Describe>(data: Data) -> Self {
		Self::custom_new(
			data,
			Describer::NAME.to_owned(),
			Describer::DESCRIPTION.map(|s| s.to_owned())
		)
	}
	
	///Describe data using the description of a separate [`&dyn DynDescribe`](DynDescribe)
	pub fn new_with_dyn_describer(data: Data, describer: &dyn DynDescribe) -> Self {
		Self::custom_new(
			data,
			describer.get_name().to_owned(),
			describer.get_description().map(|s| s.to_owned())
		)
	}
	
	
	///Changes the described data using the provided closure, while keeping the description the same
	pub fn map<NewData>(self, map_fn: impl FnOnce(Data)->NewData) -> Described<NewData> {
		Described::<NewData> {
			data: map_fn(self.data),
			name: self.name,
			description: self.description,
		}
	}
}

impl<Data: Describe> Described<Data> {
	pub fn new(data: Data) -> Self {
		Self::new_with_describer::<Data>(data)
	}
}

/**
compile-time description of a type.

Also see [`Described`] for a generic runtime description of a runtime value.
*/
pub trait Describe {
	const NAME: &'static str;
	const DESCRIPTION: Option<&'static str> = None;
}
pub trait DynDescribe {
	fn get_name(&self) -> &'static str;
	fn get_description(&self) -> Option<&'static str>;
}

impl<T: Describe> DynDescribe for T {
	fn get_name(&self) -> &'static str {
		Self::NAME
	}
	fn get_description(&self) -> Option<&'static str> {
		Self::DESCRIPTION
	}
} 