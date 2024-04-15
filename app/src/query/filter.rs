use leptos::*;
use serde::{Deserialize, Serialize};
use ff_object::describe::Described;
#[cfg(feature="ssr")]
use ffilter::{
	filter_list::FilterList,
	filter::Filter as ServerFilter,
	filter::ArgumentData,
};


#[derive(Debug, Clone,Copy, Serialize, Deserialize)]
pub enum ArgumentType {
	Bool,
}

#[cfg(feature="ssr")]
impl From<ArgumentData> for ArgumentType {
	fn from(data: ArgumentData) -> Self {
		use ArgumentData::*;
		match data {
			Bool(_) => Self::Bool,
		}
	}
}

pub type ArgumentDesc = Described<ArgumentType>;
pub type FilterDesc = Described<Vec<ArgumentDesc>>;



#[derive(Debug,Clone, PartialEq, Eq)]
pub enum ClientArgument {
	Bool(RwSignal<bool>)
}

fn client_arg_default(kind: ArgumentType) -> ClientArgument {
	use ClientArgument as CA;
	use ArgumentType as AT;
	
	match kind {
		AT::Bool => CA::Bool(RwSignal::new(false)),
	}
}

#[component]
fn BoolEditor(value: RwSignal<bool>, #[prop(default=None)] id: Option<String>) -> impl IntoView {
	view! {
		<input type="checkbox" id=id prop:checked=value on:input=move |event| {
			value.set(event_target_checked(&event));
		}/>
	}
}

#[component]
fn ArgumentUI(argument: ClientArgument, #[prop(optional, default=None)] id: Option<String>) -> impl IntoView {
	use ClientArgument::*;
	match argument {
		Bool(value) => view!{ <BoolEditor value id/> },
	}
}


#[server]
pub async fn get_filters() -> Result<Vec<FilterDesc>, ServerFnError> {
	let filters = crate::extension!(FilterList);
	
	let filter_descriptions = filters.iter_filters()
		.map(|filter| {
			let args = filter.box_clone().into_arguments().into_iter()
				.map(|arg_desc| arg_desc.map(ArgumentType::from))
				.collect();
			//Can't use new_with_dyn_describer because that needs dyn trait upcasting
			//See https://github.com/rust-lang/rust/issues/65991
			Described::custom_new(
				args,
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
	pub fn from_name(name: impl Into<String>) -> Self {
		let name = name.into();
		Self { name }
	}
	
	#[cfg(feature="ssr")]
	pub fn into_filter(self, list: FilterList) -> Result<Box<dyn ServerFilter>, ffilter::filter_list::NotFoundError> {
		let filter = list.get_by_name(&self.name)?;

		let filter = filter.box_clone();

		Ok(filter)
	}
}


#[derive(Debug,Clone, PartialEq, Eq)]
pub struct ClientFilter {
	name: String,
	arguments: Vec<ClientArgument>,
}

impl ClientFilter {
	pub fn from_name(name: impl Into<String>) -> Self {
		let name = name.into();
		Self {
			name,
			arguments: Vec::new(),
		}
	}
	
	pub fn from_description(description: &FilterDesc) -> Self {
		let arguments = description.data.iter()
			.map(|arg_desc| client_arg_default(arg_desc.data))
			.collect();
		
		Self {
			name: description.name.clone(),
			arguments,
		}
	}
}


impl From<ClientFilter> for Filter {
	fn from(client_filter: ClientFilter) -> Self {
		Self {
			name: client_filter.name,
		}
	}
}

impl From<Filter> for ClientFilter {
	fn from(filter: Filter) -> Self {
		Self {
			name: filter.name,
			arguments: Vec::new(),
		}
	}
}


#[component]
pub fn Filter(
	set: SignalSetter<ClientFilter>,
	get: Signal<ClientFilter>,
	filters: Vec<FilterDesc>,
	#[prop(into)] sub_id: String
) -> impl IntoView {
	let id = format!("filter_{sub_id}");
	
	let current = get.get();
	let desc = filters.iter().find(|filter| filter.name==current.name)
		.expect("provided ClientFilter should be from the filters list");
	let description = RwSignal::new(desc.clone());
	
	let filters2 = filters.clone();
	
	view! {
		<span>
			<select name=id.clone() id=id on:change=move |event| {
				let selected_name = event_target_value(&event);
				let filter_desc = filters.iter().find(|filter| filter.name==selected_name)
					.expect("the name can only be selected from values from this list, so it should be in this list");
				//Batch to avoid mismatch between argument data and descriptions
				batch(|| {
					set.set(ClientFilter::from_description(filter_desc));
					description.set(filter_desc.clone());
				})
			}>
				<For
					each=move || filters2.clone()
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
			
			<For
				each=move || std::iter::zip(get.get().arguments, description.get().data)
				key=|(_value, desc)| desc.name.clone()
				let:arg
			>
				<span>
					<label for=format!("arg_{}",arg.1.name)> {arg.1.name.clone()} ":" </label>
					<ArgumentUI argument=arg.0 id=format!("arg_{}",arg.1.name)/>
				</span>
			</For>
		</span>
	}
}