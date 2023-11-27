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
pub fn ObjectFields(struct_info: &'static StructInfo) -> impl IntoView {
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
pub fn ObjectValues<Item: Struct, 'item>(item: &'item Item) -> impl IntoView {
	item.iter_fields().map(|field| {
		view! {
			<span class="object_value"><Reflected value=field/></span>
		}
	}).collect::<Vec<_>>()
}

#[component]
pub fn ObjectList<Item: Struct + Typed + Clone, Str: AsRef<str>>(
	#[prop(into)] items: MaybeSignal<Vec<Item>>,
	get_id: fn(&Item)->i32,
	list_class: Str
) -> impl IntoView {
	view! {
		<ul class={format!("object_list {}", list_class.as_ref())}>
			<ObjectFields struct_info={struct_info::<Item>()} />
			<For
				each = move || items.get().into_iter()
				key = get_id
				let:item
			>
				<A class="object_value_list" href={get_id(&item).to_string()}>
					<ObjectValues item = &item/>
				</A>
			</For>
		</ul>
	}
}

///An <ObjectList> styled to be a table
#[component]
pub fn ObjectTable<Item: Struct + Typed + Clone>(
	#[prop(into)] items: MaybeSignal<Vec<Item>>,
	get_id: fn(&Item)->i32
) -> impl IntoView {
	view! { <ObjectList items get_id list_class="object_table" /> }
}
