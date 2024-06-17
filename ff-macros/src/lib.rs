use proc_macro::TokenStream;
use syn::{
	parse_macro_input,
	DeriveInput
};
use proc_macro_error::proc_macro_error;

mod object;
mod field_list;


/**
Generates an impl for [ff_object::Object](../ff_object/traits/trait.Object.html).

Id field selection:
1. first field named "id"

Object name selection:
1. containing module
2. crate name
*/
#[proc_macro_error]
#[proc_macro_derive(Object)]
pub fn object_derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	object::ObjectDerive::from(input)
		.generate()
		.into()
}


/**
Generates an impl for [`ff_object::fields::FieldListable`](../ff_object/fields/trait.FieldListable.html)`<``dyn bevy_reflect::Reflect``>`.

Attributes:

`field_list` on struct:
-	`lists`: explicitly list the types to generate [`FieldListable`](../ff_object/fields/trait.FieldListable.html) impls for. Defaults to `dyn bevy_reflect::Reflect`

See the tests for example usage.

*/
#[proc_macro_error]
#[proc_macro_derive(FieldList, attributes(field_list))]
pub fn field_list_derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	field_list::FieldListDerive::from(input)
		.generate()
		.into()
}