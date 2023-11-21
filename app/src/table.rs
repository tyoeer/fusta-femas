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
pub fn StructFields<Item: Struct, 'item>(item: &'item Item) -> impl IntoView {
	item.iter_fields().map(|field| {
		view! {
			<span class="table_cell"><Reflected value=field/></span>
		}
	}).collect::<Vec<_>>()
}

#[component]
pub fn Table<Item: Struct + Typed + Clone>(#[prop(into)] items: MaybeSignal<Vec<Item>>, get_id: fn(&Item)->i32) -> impl IntoView {
	view! {
		<ul class="table">
			<TableHeader struct_info={struct_info::<Item>()} />
			<For
				each = move || items.get().into_iter()
				key = get_id
				let:item
			>
				<A class="table_row" href={get_id(&item).to_string()}>
					<StructFields item = &item/>
				</A>
			</For>
		</ul>
	}
}
