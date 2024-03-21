use leptos::*;
use serde::{Deserialize, Serialize};
use crate::utils;

pub mod filter;
use filter::ClientFilter;

#[derive(Debug, Default, Clone, PartialEq,Eq, Serialize, Deserialize)]
pub struct Query {
	filter: Option<ClientFilter>,
}

impl Query {
	pub fn from_filter_name(name: impl Into<String>) -> Self {
		Self {
			filter: Some(ClientFilter::from_name(name.into())),
		}
	}
	
	pub fn from_filter_signal(filter_signal: RwSignal<Option<RwSignal<ClientFilter>>>) -> Self {
		match filter_signal.get() {
			None => Self {
				filter: None
			},
			Some(filter_signal) => {
				let filter = filter_signal.get();
				Self {
					filter: Some(filter)
				}
			}
		}
	}
	
	pub fn into_filter(self) -> Option<ClientFilter> {
		self.filter
	}
}

///Wrapper around Query with [std::fmt::Display]/[ToString] and [std::str::FromStr] impl based on serde_json. Used to put a (search) query in a browser query.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct QueryString {
	pub query: Query,
}

impl std::fmt::Display for QueryString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let maybe_serialized = serde_json::to_string(&self.query);
		let serialized = maybe_serialized.map_err(|err| {
			tracing::error!(?err, "Error serializing query");
			std::fmt::Error
		})?;
		write!(f, "{serialized}")
	}
}

impl std::str::FromStr for QueryString {
	type Err = serde_json::Error;
	
	fn from_str(source: &str) -> Result<Self, Self::Err> {
		let query = serde_json::from_str::<Query>(source)?;
		Ok(Self { query })
	}
}

impl From<Query> for QueryString {
	fn from(query: Query) -> Self {
		Self { query }
	}
}

impl From<QueryString> for Query {
	fn from(query_string: QueryString) -> Self {
		query_string.query
	}
}

#[component]
pub fn QueryUI(#[prop(into)] on_search: Callback<Query>, pending: Signal<bool>) -> impl IntoView {
	let again = RwSignal::new(false);
	
	let button_name = move || {
		if pending.get() {
			"searching...".to_owned()
		} else {
			format!("search{}", if again.get() {" again"} else {""} )
		}
	};
	
	let filter: RwSignal<Option<RwSignal<ClientFilter>>> = RwSignal::new(None);
	
	let filter_ui = move |filters| {
		match filter.get() {
			Some(filter_sig) => {
				let (get, set) = filter_sig.split();
				view! {
					<filter::Filter get=get.into() set=set.into() filters sub_id="" />
				}.into_view()
			},
			None => ().into_view(),
		}
	};
	
	view! {
		<div class="search">
			<div class="search_parameters">
				<div class="search_parameter">
					<utils::AwaitOk future=filter::get_filters let:filters>
						<utils::CloneSignal base=filters let:filters_signal>
							<label for="filter_enable">filter</label>
							<input type="checkbox" id="filter_enable" on:input=move |event| {
								if event_target_checked(&event) {
									let default = filters_signal.get().first().expect("the server should have at least 1 filter").clone();
									filter.set(Some(RwSignal::new(ClientFilter::from_description(default))));
								} else {
									filter.set(None);
								}
							}/>
							{ move || filter_ui(filters_signal.get()) }
						</utils::CloneSignal>
					</utils::AwaitOk>
				</div>
			</div>
			
			<button
				disabled=pending
				on:click = move |_event| {
					let query = Query::from_filter_signal(filter);
					again.set(true);
					on_search.call(query.clone());
				}
			>
				{button_name}
			</button>
		</div>
	}
}

