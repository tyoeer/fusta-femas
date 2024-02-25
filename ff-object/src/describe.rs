/**
Generic runtime data that is described at runtime using a name and an optional description.

Also see [`Describe`] for a compile-time description of a type.
*/
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