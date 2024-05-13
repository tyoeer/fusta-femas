use proc_macro2::{TokenStream, Span};
use quote::{quote, quote_spanned};
use syn::{DeriveInput, Ident, Type, TypePath, parse2, punctuated::Punctuated, spanned::Spanned};
use proc_macro_error::abort;


#[derive(Clone)]
pub struct Field {
	name: Ident,
	ty: Type,
	span: Span,
}

pub struct List {
	r#type: Type,
	fields: Vec<Field>,
}

impl List {
	pub fn new(ty: Type) -> Self {
		Self {
			r#type: ty,
			fields: Vec::new(),
		}
	}
	
	pub fn generate_impl(self, crate_path: &TypePath, struct_name: &Ident) -> TokenStream {
		let field_type = self.r#type;
		let fields = self.fields.into_iter().map(|field| {
			let Field {mut name, ty, span} = field;
			let name_str = name.to_string();

			//Use type span to put errors of the field not implementing Reflect on the type
			name.set_span(ty.span());
			let get = quote_spanned! {ty.span()=> &obj.#name};
			let get_mut = quote_spanned! {ty.span()=> &mut obj.#name};
			// Don't use the ff_object::dyn_field! macro because it loses span info and puts the error on the macro instead of the field
			quote_spanned! {span=> 
				#crate_path::fields::DynField::<#struct_name>::new(
					std::borrow::Cow::Borrowed(#name_str),
					|obj| #get,
					|obj| #get_mut,
				),
			}
		});

		quote! {

			impl #crate_path::fields::FieldListable<#field_type> for #struct_name {
				fn iter_fields() -> impl Iterator<Item = &'static (impl #crate_path::fields::Field<Object=Self, FieldType=#field_type> + 'static) > {
					//const item to prevent duplication
					const FIELDS: &[#crate_path::fields::DynField<#struct_name>] = &[
						#(#fields)*
					];

					FIELDS.iter()
				}
			}
		}
	}
}

fn default_list_type() -> Type {
	parse2(quote!{ dyn bevy_reflect::Reflect }).expect("hardcoded type should be valid")
}

impl Default for List {
	fn default() -> Self {
		Self {
			r#type: default_list_type(),
			fields: Vec::new(),
		}
	}
}

pub struct FieldListDerive {
	struct_name: Ident,
	crate_path: TypePath,
	lists: Vec<List>,
}

impl FieldListDerive {
	pub fn new(struct_name: Ident, lists: Vec<List>) -> Self {
		Self {
			struct_name,
			lists,
			crate_path: Self::default_trait_path(),
		}
	}
	fn default_trait_path() -> TypePath {
		parse2(quote!{ ::ff_object }).expect("hardcoded path should be valid")
	}
	
}

impl From<DeriveInput> for FieldListDerive {
	fn from(derive_input: DeriveInput) -> Self {
		let DeriveInput {ident: struct_name, data, ..} = derive_input;
		
		//Attribute
		
		let mut lists = vec![ List::default() ];
		
		//Fields
		
		let struct_data = match data {
			syn::Data::Struct(struct_data) => struct_data,
			syn::Data::Enum(enum_data) => abort!(enum_data.enum_token, "enums aren't supported"),
			syn::Data::Union(union_data) => abort!(union_data.union_token, "unions aren't supported"),
		};
		
		let fields = match struct_data.fields {
			syn::Fields::Named(fields) => fields.named,
			syn::Fields::Unnamed(fields) => fields.unnamed,
			syn::Fields::Unit => Punctuated::new(),
		};
		
		let mut counter = 0;
		for field in fields {
			let span = field.span();
			let name = if let Some(ident) = field.ident {
				ident
			} else {
				counter += 1;
				Ident::new(counter.to_string().as_ref(), field.span())
			};
			
			let field = Field {
				name,
				span,
				ty: field.ty,
			};
			
			for list in &mut lists {
				list.fields.push(field.clone());
			}
		}
		
		Self::new(struct_name, lists)
	}
}

impl FieldListDerive {
	pub fn generate(self) -> TokenStream {
		let Self {
			struct_name,
			crate_path,
			lists
		} = self;
		
		lists.into_iter()
			.map(|list| list.generate_impl(&crate_path, &struct_name))
			.collect()
	}
	

}