use leptos::*;
use serde::{Deserialize, Serialize};
use ff_object::describe::Described;
#[cfg(feature="ssr")]
use ffilter::filter_list::FilterList;
use crate::{
	utils,
	extension,
};


pub type Argument = Described<()>;
pub type FilterData = Described<Vec<Argument>>;

#[server]
pub async fn get_filters() -> Result<Vec<Described<()>>, ServerFnError> {
	let filters = extension!(FilterList);
	
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
pub struct Filter {
	name: String,
}

impl Filter {
	pub fn from_name(name: String) -> Self {
		Self { name }
	}
}


#[component]
pub fn Filter(filter: RwSignal<Filter>, #[prop(into)] sub_id: String) -> impl IntoView {
	let id = format!("filter_{sub_id}");
	
	let filter_signal = filter;
	
	view! {
		<div>
			<select name=id.clone() id=id on:change=move |event| {
				let value = event_target_value(&event);
				filter_signal.update(|filter| filter.name = value);
			}>
				<utils::AwaitOk future=get_filters let:filters>
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
									selected=move || filter_signal.get().name==name_clone
								>
									{filter_data.name}
								</option>
							}
						}
					</For>
				</utils::AwaitOk>
			</select>
		</div>
	}
}