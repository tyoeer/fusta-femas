use leptos::*;
use crate::utils;
use serde::{Deserialize, Serialize};
use ff_object::describe::Described;
use entities::prelude::tag;
#[cfg(feature="ssr")]
use ffilter::{
	filter_list::FilterList,
	filter::Filter as ServerFilter,
	filter::ArgumentData,
};


#[derive(Debug, Clone,Copy, Serialize, Deserialize)]
pub enum ArgumentType {
	Bool,
	Tag,
}

#[cfg(feature="ssr")]
impl From<ArgumentData> for ArgumentType {
	fn from(data: ArgumentData) -> Self {
		use ArgumentData::*;
		match data {
			Bool(_) => Self::Bool,
			Tag(_) => Self::Tag,
		}
	}
}

pub type ArgumentDesc = Described<ArgumentType>;
pub type FilterDesc = Described<Vec<ArgumentDesc>>;


#[derive(Debug,Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Argument {
	Bool(bool),
	Tag(tag::Ref)
}


#[cfg(feature="ssr")]
impl From<Argument> for ArgumentData {
	fn from(arg: Argument) -> Self {
		use ArgumentData as AD;
		use Argument as A;
		match arg {
			A::Bool(value) => AD::Bool(value),
			A::Tag(value) => AD::Tag(value),
		}
	}
}

impl From<ClientArgument> for Argument {
	fn from(ca: ClientArgument) -> Self {
		use ClientArgument as CA;
		use Argument as A;
		match ca {
			CA::Bool(sig) => A::Bool(sig.get()),
			CA::Tag(sig) => A::Tag(sig.get()),
		}
	}
}

#[derive(Debug,Clone, PartialEq, Eq)]
pub enum ClientArgument {
	Bool(RwSignal<bool>),
	Tag(RwSignal<tag::Ref>),
}

fn client_arg_default(kind: ArgumentType, default_tag: Option<tag::Ref>) -> ClientArgument {
	use ClientArgument as CA;
	use ArgumentType as AT;
	
	match kind {
		AT::Bool => CA::Bool(RwSignal::new(false)),
		AT::Tag => CA::Tag(RwSignal::new(default_tag.expect("there should exist a tag to select"))),
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
fn TagEditor(value: RwSignal<tag::Ref>, #[prop(default=None)] id: Option<String>) -> impl IntoView {
	view! {
		<select id=id on:change=move |event| {
			let id_str = event_target_value(&event);
			let id = id_str.parse::<i32>().expect("option value should have been set to a valid i32");
			value.set(tag::Ref::new(id));
			
		}>
			<utils::AwaitOk future=crate::tag::search::all_tags let:tags>
				<For
					each=move || tags.clone()
					key=|tag| tag.id
					let:tag
				>
					<option
						value=tag.id
						selected=move || value.get().id()==tag.id
					>
						{tag.title}
					</option>
				</For>
			</utils::AwaitOk>
		</select>
	}
}

#[component]
fn ArgumentUI(argument: ClientArgument, #[prop(optional, default=None)] id: Option<String>) -> impl IntoView {
	use ClientArgument::*;
	match argument {
		Bool(value) => view!{ <BoolEditor value id/> },
		Tag(value) => view!{ <TagEditor value id/> },
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

#[cfg(feature="ssr")]
#[derive(Debug, thiserror::Error)]
pub enum FromFilterError {
	#[error("Server does not know filter: {0}")]
	ListNotFound(#[from] ffilter::filter_list::NotFoundError),
	#[error("Wrong arguments for filter: {0}")]
	ArgumentError(#[from] ffilter::filter::ArgumentError)
}

#[derive(Debug,Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Filter {
	name: String,
	#[serde(default)] //Default transport format errors on empty Vec
	arguments: Vec<Argument>,
}

impl Filter {
	pub fn from_name(name: impl Into<String>) -> Self {
		let name = name.into();
		Self {
			name,
			arguments: Vec::new(),
		}
	}
	
	#[cfg(feature="ssr")]
	pub fn into_filter(self, list: FilterList) -> Result<Box<dyn ServerFilter>, FromFilterError> {
		let builder = list.get_builder_by_name(&self.name)?;
		
		let arguments = self.arguments.into_iter()
			.map(ArgumentData::from)
			.collect();
		
		
		let filter = (builder.data)(arguments)?;
		
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
		let default_tag = Some(tag::Ref::new(1));
		let arguments = description.data.iter()
			.map(|arg_desc| client_arg_default(arg_desc.data, default_tag.clone()))
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
			arguments: client_filter.arguments.into_iter().map(|arg| arg.into()).collect()
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
pub fn FilterUI(
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