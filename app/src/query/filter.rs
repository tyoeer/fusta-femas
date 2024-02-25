use leptos::*;
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

#[component]
pub fn Filter(#[prop(into)] sub_id: String) -> impl IntoView {
	let id = format!("filter_{sub_id}");
	
	view! {
		<div>
			<select name=id.clone() id=id>
				<utils::AwaitOk future=get_filters let:filters>
					<For
						each=move || filters.clone()
						key=|filter| filter.name.clone()
						let:filter
					>
						<option value=filter.name.clone()> {filter.name} </option>
					</For>
				</utils::AwaitOk>
			</select>
		</div>
	}
}