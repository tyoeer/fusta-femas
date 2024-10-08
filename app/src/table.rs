/*!

Components for displaying stuff that implements [`bevy_reflect::Reflect`], usually structs.

Terminology:
- Object: A displayable struct
- Fields: the names of the object's struct fields
- Values: the actual runtime values in the object
- List: When an enclosing component is also rendered

*/

use std::marker::PhantomData;

use leptos::*;
use leptos_router::A;
use bevy_reflect::prelude::*;

use ff_object::fields::{FieldListable, Field};
use entities::prelude as entities;

/**
Turns a reflected value into a string to display

## Supported values

- [`String`]
- [`Option`]`<`[`String`]`>`
- [`i32`]
- [`Option`]`<`[`i32`]`>`
- [`bool`]
- [`fetch::Status`](entities::fetch::Status)

*/
pub fn reflect_to_string(value: &dyn Reflect) -> String {
	if let Some(str) = value.downcast_ref::<String>() {
		str.clone()
	} else if let Some(maybe_str) = value.downcast_ref::<Option<String>>() {
		match maybe_str {
			Some(str) => str.clone(),
			None => "".to_owned(),
		}
	} else if let Some(i) = value.downcast_ref::<i32>() {
		i.to_string()
	} else if let Some(maybe_int) = value.downcast_ref::<Option<i32>>() {
		match maybe_int {
			Some(int) => int.to_string(),
			None => "".to_owned(),
		}
	} else if let Some(boolean) = value.downcast_ref::<bool>() {
		(if *boolean {"yes"} else {"no"}).to_owned()
	} else if let Some(status) = value.downcast_ref::<entities::fetch::Status>() {
		status.to_string()
	} else if let Some(date) = value.downcast_ref::<entities::time_fields::Date>() {
		date.to_string()
	} else if let Some(time) = value.downcast_ref::<entities::time_fields::Time>() {
		time.to_string()
	} else if let Some(date_time) = value.downcast_ref::<entities::time_fields::PrimitiveDateTime>() {
		date_time.to_string()
	} else if let Some(option_time) = value.downcast_ref::<entities::time_fields::OptionTime>() {
		option_time.to_string()
	} else {
		tracing::error!("Don't know how to display a {}", value.reflect_type_path());
		"🤷".to_owned()
	}
}

//TODO move into style or make a setting
const SHORTENED_MAX_SIZE: usize = 55;

/**
Displays a reflected value.

See [`reflect_to_string()`](reflect_to_string) for supported values

*/
#[component]
pub fn Reflected<'a>(value: &'a dyn Reflect, #[prop(default = false)] short: bool) -> impl IntoView {
	let reflected = reflect_to_string(value);
	if !short {return reflected.into_view()};
	
	let trimmed = reflected.trim();
	let first_line = match trimmed.split_once('\n') {
		None => {
			trimmed
		},
		Some((first_line, _other_lines)) => {
			first_line
		},
	};
	
	let mut shortened = first_line.chars().take(SHORTENED_MAX_SIZE).collect::<String>();
	if shortened.len() != trimmed.len() {
		if shortened.len() == first_line.len() {
			//This is the first line, preserve some whitespace
			shortened.push(' ');
		}
		shortened.push_str("...");
		view! {
			<span title=reflected>
				{shortened}
			</span>
		}.into_view()
	} else {
		shortened.into_view()
	}
	
}




//TODO find way to get rid of PhantomData requirements
#[component]
pub fn ObjectFieldList<Object: FieldListable<dyn Reflect>>(
	#[prop(optional)] _ignore: PhantomData<Object>,
	#[prop(optional)] adds: ObjectValueAdds<Object>,
) -> impl IntoView {
	view! {
		<li class="object_field_list">
			{
				Object::iter_fields().map(|field| {
					view! {<span class="object_field">{field.name()}</span>}
				}).collect::<Vec<_>>()
			}
			{
				adds.into_iter().map(|add| view!{
					<span class="object_field">{add.0}</span>
				}).collect::<Vec<_>>()
			}
		</li>
	}
}

#[component]
pub fn ObjectValues<Object: FieldListable<dyn Reflect>, 'object>(object: &'object Object) -> impl IntoView {
	object.iter_values().map(|field| {
		view! {
			<span class="object_value"><Reflected value=field short=true/></span>
		}
	}).collect::<Vec<_>>()
}

/**
The tuple components are:
- `&'static str`: which field the overwrite
- `bool`: false if only the value should be overwritten, true if the entire list entry should be overwritten
- `fn(&Object) -> View`: function taking in the object whose fields are being displayed, returning what to render as value
*/
pub type FieldValueOverwrites<Object> = Vec<(&'static str, bool, fn(&Object) -> View)>;

#[component]
pub fn ObjectFieldValues<Object: FieldListable<dyn Reflect> + 'static>(
	#[prop(into)]
	object: MaybeSignal<Object>,
	#[prop(optional)]
	overloads: FieldValueOverwrites<Object>,
) -> impl IntoView {
	move || object.with(|object| {
		object.iter_field_values().map(|(field, value)| {
			if let Some(overload) = overloads
				.iter()
				.find( |(overload_field, _, _)| field == *overload_field )
			{
				if overload.1 {
					overload.2(object)
				} else {
					view! {
						<li class="object_fieldvalue">
							<span class="object_field"> {field} </span>
							<span class="object_value"> {overload.2(object)} </span>
						</li>
					}.into_view()
				}
			} else {
				view! {
					<li class="object_fieldvalue">
						<span class="object_field"> {field} </span>
						<span class="object_value"> <Reflected value/> </span>
					</li>
				}.into_view()
			}
		}).collect::<Vec<_>>()
	})
}

#[component]
pub fn ObjectFieldValueList<Object: FieldListable<dyn Reflect> + 'static>(
	#[prop(into)]
	object: MaybeSignal<Object>,
	#[prop(optional)]
	overloads: FieldValueOverwrites<Object>,
) -> impl IntoView {
	view! {
		<ul class="object_fieldvalue_list">
			<ObjectFieldValues object overloads/>
		</ul>
	}
}

/**
The tuple components are:
- `&'static str`: name of field
- `fn(&Object) -> View`: function taking in the object whose fields are being displayed, returning what to render for this field
*/
pub type ObjectValueAdds<Object> = Vec<(&'static str, fn(&Object) -> View)>;

#[component]
pub fn ObjectLinkValues<Object: FieldListable<dyn Reflect> + Clone + ff_object::Object + 'static>(
	#[prop(into)] items: MaybeSignal<Vec<Object>>,
	#[prop(optional)] adds: ObjectValueAdds<Object>,
) -> impl IntoView {
	let adds = store_value(adds);
	view! {
		<For
			each = move || items.get().into_iter()
			key = Object::get_id
			let:object
		>
			<A class="object_value_list" href={ format!(
				"/{}/{}",
				Object::get_object_name(),
				object.get_id()
			)} >
				<ObjectValues object = &object/>
				<For
					each = move || adds.get_value().into_iter()
					key = |add| add.0
					let:addition
				>
					<span class="object_value">
					<div>
						{addition.1(&object)}
						</div>
					</span>
				</For>
			</A>
		</For>
	}
}

///A table of objects where each row is a link
#[component]
pub fn ObjectTable<Object: FieldListable<dyn Reflect> + Clone + ff_object::Object + 'static>(
	#[prop(into)] items: MaybeSignal<Vec<Object>>,
	#[prop(optional)] adds: ObjectValueAdds<Object>,
) -> impl IntoView {
	view! {
		<ul class="object_list object_table">
			<ObjectFieldList<Object> adds=adds.clone() />
			<ObjectLinkValues items adds />
		</ul>
	}
}
