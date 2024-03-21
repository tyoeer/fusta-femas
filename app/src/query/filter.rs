use leptos::*;
use serde::{Deserialize, Serialize};
use ff_object::describe::Described;
#[cfg(feature="ssr")]
use ffilter::{
	filter_list::FilterList,
	filter::Filter,
};


pub type Argument = Described<()>;
pub type FilterData = Described<Vec<Argument>>;

#[server]
pub async fn get_filters() -> Result<Vec<Described<()>>, ServerFnError> {
	let filters = crate::extension!(FilterList);
	
	let filter_descriptions = filters.iter_filters()
		.map(|filter| {
			//Can't use new_with_dyn_describer because that needs dyn trait upcasting
			//See https://github.com/rust-lang/rust/issues/65991
			Described::custom_new(
				(),
				filter.get_name().to_owned(),
				filter.get_description().map(|d| d.to_owned()),
			)
		})
		.collect();
	
	Ok(filter_descriptions)
}


#[derive(Debug,Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientFilter {
	name: String,
}

impl ClientFilter {
	pub fn from_name(name: impl Into<String>) -> Self {
		let name = name.into();
		Self { name }
	}
	
	pub fn from_description(description: Described<()>) -> Self {
		Self::from_name(description.name)
	}
	
	#[cfg(feature="ssr")]
	pub fn into_filter(self, list: FilterList) -> Result<Box<dyn Filter>, ffilter::filter_list::NotFoundError> {
		let filter = list.get_by_name(&self.name)?;
		
		let filter = filter.box_clone();
		
		Ok(filter)
	}
}


#[component]
pub fn Filter(
	set: SignalSetter<ClientFilter>,
	get: Signal<ClientFilter>,
	filters: Vec<Described<()>>,
	#[prop(into)] sub_id: String
) -> impl IntoView {
	let id = format!("filter_{sub_id}");
	
	view! {
		<span>
			<select name=id.clone() id=id on:change=move |event| {
				let value = event_target_value(&event);
				let mut filter = get.get();
				filter.name = value;
				set.set(filter);
			}>
				<For
					each=move || filters.clone()
					key=|filter| filter.name.clone()
					let:filter_data
				>
					{
						let name_clone = filter_data.name.clone();
						view! {
							<option
								value=filter_data.name.clone()
								selected=move || get.get().name==name_clone
							>
								{filter_data.name}
							</option>
						}
					}
				</For>
			</select>
		</span>
	}
}