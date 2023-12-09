/*!

Components for displaying stuff that implements [`bevy_reflect::Reflect`], usually structs.

Terminology:
- Object: A displayable struct
- Fields: the names of the object's struct fields
- Values: the actual runtime values in the object
- List: When an enclosing component is also rendered

*/

use leptos::*;
use leptos_router::A;
use bevy_reflect::{
	prelude::*,
	Typed, TypeInfo, StructInfo,
};

pub fn struct_info<Type: Struct + Typed>() -> &'static StructInfo {
	if let TypeInfo::Struct(info) = Type::type_info() {
		info
	} else {
		unreachable!("Type guard should have guaranteed \"{}\" was a struct", Type::type_path());
	}
}

/**
Displays a reflected value.

## Supported values

- [`String`](std::string::String)
- [`i32`](std::i32)

*/
#[component]
pub fn Reflected<'a>(value: &'a dyn Reflect) -> impl IntoView {
	if let Some(str) = value.downcast_ref::<String>() {
		 str.clone()
	} else if let Some(i) = value.downcast_ref::<i32>() {
		 format!("{i}")
	} else {
		tracing::error!("Don't know how to display a {}", value.reflect_type_path());
		"🤷".to_owned()
	}
}

#[component]
pub fn ObjectFieldList(struct_info: &'static StructInfo) -> impl IntoView {
	view! {
		<li class="object_field_list">
			{
				struct_info.field_names().iter().map(|name| {
					view! {<span class="object_field">{*name}</span>}
				}).collect::<Vec<_>>()
			}
		</li>
	}
}

#[component]
pub fn ObjectValues<Object: Struct, 'object>(object: &'object Object) -> impl IntoView {
	object.iter_fields().map(|field| {
		view! {
			<span class="object_value"><Reflected value=field/></span>
		}
	}).collect::<Vec<_>>()
}

#[component]
pub fn ObjectFieldValues<Object: Struct + Typed, 'object>(object: &'object Object) -> impl IntoView {
	let struct_info = struct_info::<Object>();
	object.iter_fields().zip(struct_info.field_names()).map(|(value, field)| {
		view! {
			<li class="object_fieldvalue">
				<span class="object_field"> {*field} </span>
				<span class="object_value"> <Reflected value/> </span>
			</li>
		}
	}).collect::<Vec<_>>()
}

#[component]
pub fn ObjectFieldValueList<Object: Struct + Typed, 'object>(object: &'object Object) -> impl IntoView {
	view! {
		<ul class="object_fieldvalue_list">
			<ObjectFieldValues object />
		</ul>
	}
}

#[component]
pub fn ObjectLinkValues<Object: Struct + Typed + Clone>(
	#[prop(into)] items: MaybeSignal<Vec<Object>>,
	get_id: fn(&Object)->i32,
) -> impl IntoView {
	view! {
		<For
			each = move || items.get().into_iter()
			key = get_id
			let:object
		>
			<A class="object_value_list" href={get_id(&object).to_string()}>
				<ObjectValues object = &object/>
			</A>
		</For>
	}
}

///A table of objects where each row is a link
#[component]
pub fn ObjectTable<Object: Struct + Typed + Clone>(
	#[prop(into)] items: MaybeSignal<Vec<Object>>,
	get_id: fn(&Object)->i32
) -> impl IntoView {
	view! {
		<ul class="object_list object_table">
			<ObjectFieldList struct_info={struct_info::<Object>()} />
			<ObjectLinkValues items get_id/>
		</ul>
	}
}
