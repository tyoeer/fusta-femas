use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, TypePath, parse2};
use proc_macro_error::abort;

pub struct ObjectDerive {
	id_field: Ident,
	struct_name: Ident,
	trait_path: TypePath,
}

impl ObjectDerive {
	pub fn new(struct_name: Ident, id_field: Ident) -> Self {
		Self {
			struct_name,
			id_field,
			trait_path: Self::default_trait_path(),
		}
	}
	fn default_trait_path() -> TypePath {
		parse2(quote!{ ::ff_object::Object }).expect("hardcoded path should be valid")
	}
}

impl From<DeriveInput> for ObjectDerive {
	fn from(di: DeriveInput) -> Self {
		let struct_name = di.ident.to_owned();
		let struct_data = match &di.data {
			syn::Data::Struct(struct_data) => struct_data,
			syn::Data::Enum(enum_data) => abort!(enum_data.enum_token, "enums aren't supported"),
			syn::Data::Union(union_data) => abort!(union_data.union_token, "unions aren't supported"),
		};
		//Find id
		let id = match &struct_data.fields {
			syn::Fields::Named(fields) => {
				'id_field: {
					for field in &fields.named {
						let ident = field.ident.as_ref().expect("we just checked this to be named fields");
						if ident == "id" {
							break 'id_field ident;
						}
					}
					abort!(fields.named, "no id field");
				}
			},
			syn::Fields::Unnamed(_) => todo!(),
			syn::Fields::Unit => abort!(di, "unit struct has no possible id field"),
		};
		
		
		
		Self::new(struct_name, id.to_owned())
	}
}

impl ObjectDerive {
	pub fn generate(self) -> TokenStream {
		let trait_path = self.trait_path;
		let struct_name = self.struct_name;
		let id_field = self.id_field;
		quote! {
			impl #trait_path for #struct_name {
				fn get_id(&self) -> i32 {
					self.#id_field
				}
				
				fn get_object_name() -> &'static str {
					let maybe = ::core::module_path!().rsplit_once("::");
					match maybe {
						Some((_, module)) => module,
						None => ::core::module_path!()
					}
				}
			}
		}
	}
}