use proc_macro::TokenStream;
use syn::{
	parse_macro_input,
	DeriveInput
};
use proc_macro_error::proc_macro_error;

mod object;
mod field_list;


/**
Generates an impl for [ff_objects::Object].

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
Generates an impl for [ff_objects::fields::FieldListable].
*/
#[proc_macro_error]
#[proc_macro_derive(FieldList)]
pub fn field_list_derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	field_list::FieldListDerive::from(input)
		.generate()
		.into()
}