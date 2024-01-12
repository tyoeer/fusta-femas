use proc_macro2::{TokenStream, Span};
use quote::{quote, quote_spanned};
use syn::{DeriveInput, Ident, Type, TypePath, parse2, punctuated::Punctuated, spanned::Spanned};
use proc_macro_error::abort;

pub struct Field {
	name: Ident,
	ty: Type,
	span: Span,
}

pub struct FieldListDerive {
	struct_name: Ident,
	fields: Vec<Field>,
	crate_path: TypePath,
}

impl FieldListDerive {
	pub fn new(struct_name: Ident, fields: Vec<Field>) -> Self {
		Self {
			struct_name,
			fields,
			crate_path: Self::default_trait_path(),
		}
	}
	fn default_trait_path() -> TypePath {
		parse2(quote!{ ::ff_object }).expect("hardcoded path should be valid")
	}
}

impl From<DeriveInput> for FieldListDerive {
	fn from(di: DeriveInput) -> Self {
		let struct_name = di.ident.to_owned();
		let struct_data = match di.data {
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
		let fields: Vec<Field> = fields.into_iter()
			.map(|field| {
				let span = field.span();
				let name = if let Some(ident) = field.ident {
					ident
				} else {
					counter += 1;
					Ident::new(counter.to_string().as_ref(), field.span())
				};
				
				Field {
					name,
					span,
					ty: field.ty,
				}
			})
			.collect();
		
		Self::new(struct_name, fields)
	}
}

impl FieldListDerive {
	pub fn generate(self) -> TokenStream {
		let crate_path = self.crate_path;
		let struct_name = self.struct_name;
		
		let field_type = quote!{ dyn bevy_reflect::Reflect };
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
				fn iter_fields() -> impl Iterator<Item = impl #crate_path::fields::Field<Object=Self, FieldType=#field_type> > {
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