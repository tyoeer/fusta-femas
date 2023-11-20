use leptos::*;
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

#[component]
pub fn Reflected<'a>(value: &'a dyn Reflect) -> impl IntoView {
	if let Some(str) = value.downcast_ref::<String>() {
		 str.clone()
	} else if let Some(i) = value.downcast_ref::<i32>() {
		 format!("{i}")
	} else {
		"🤷".to_owned()
	}
}

#[component]
pub fn TableHeader(struct_info: &'static StructInfo) -> impl IntoView {
	view! {
		<li class="table_row">
			{
				struct_info.field_names().iter().map(|name| {
					view! {<span class="table_cell">{*name}</span>}
				}).collect::<Vec<_>>()
			}
		</li>
	}
}

#[component]
pub fn TableRow<Item: Struct, 'item>(item: &'item Item) -> impl IntoView {
	item.iter_fields().map(|field| {
		view! {
			<span class="table_cell"><Reflected value=field/></span>
		}
	}).collect::<Vec<_>>()
}

