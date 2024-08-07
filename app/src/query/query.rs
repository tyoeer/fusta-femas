use leptos::*;
use serde::{Deserialize, Serialize};
use crate::utils;

use super::{ClientFilter, Filter, FilterUI, filter::get_filters};


///Condensed query type for transport between server and client
#[derive(Debug, Default, Clone, PartialEq,Eq, Serialize, Deserialize)]
pub struct Query {
	filter: Option<Filter>,
}

impl Query {
	pub fn from_filter_name(name: impl Into<String>) -> Self {
		Self {
			filter: Some(Filter::from_name(name.into())),
		}
	}
	
	pub fn into_filter(self) -> Option<Filter> {
		self.filter
	}
}


///Query type with signals in its innards so it can easily be hooked up into editor UI
#[derive(Debug, Default, Clone, PartialEq,Eq)]
pub struct ClientQuery {
	filter: RwSignal<Option<RwSignal<ClientFilter>>>,
}

impl ClientQuery {
	pub fn get_filter_signal(&self) -> RwSignal<Option<RwSignal<ClientFilter>>> {
		self.filter
	}
	
	
	pub fn into_query(&self) -> Query {
		let filter = self.filter.get().map(|filter_sig| filter_sig.get().into());
		Query {
			filter,
		}
	}
}

impl From<ClientQuery> for Query {
	fn from(client_query: ClientQuery) -> Self {
		client_query.into_query()
	}
}

impl From<Query> for ClientQuery {
	fn from(query: Query) -> Self {
		let filter = RwSignal::new(query.filter.map(|filter| RwSignal::new(filter.into())));
		Self {
			filter,
		}
	}
}


/**
Wrapper around [Query] with [std::fmt::Display]/[ToString] and [std::str::FromStr] impl based on serde_json.
Used to put a (search) query in a browser query.
*/
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
pub fn QueryUI(#[prop(into)] on_search: Callback<Query>, pending: Signal<bool>, #[prop(default=None)] default: Option<Query>) -> impl IntoView {
	let again = RwSignal::new(false);
	
	let button_name = move || {
		if pending.get() {
			"searching...".to_owned()
		} else {
			format!("search{}", if again.get() {" again"} else {""} )
		}
	};
	
	let client_query = default.map(ClientQuery::from).unwrap_or_default();
	let filter = client_query.get_filter_signal();
	
	let filter_ui = move |filters| {
		match filter.get() {
			Some(filter_sig) => {
				let (get, set) = filter_sig.split();
				view! {
					<FilterUI get=get.into() set=set.into() filters sub_id="" />
				}.into_view()
			},
			None => ().into_view(),
		}
	};
	
	view! {
		<div class="search">
			<div class="search_parameters">
				<div class="search_parameter">
					<utils::AwaitOk future=get_filters let:filters>
						<utils::CloneSignal base=filters let:filters_signal>
							<label for="filter_enable">filter</label>
							<input type="checkbox" id="filter_enable" prop:checked=filter.get().is_some() on:input=move |event| {
								if event_target_checked(&event) {
									let filters = filters_signal.get();
									let default = filters.first().expect("the server should have at least 1 filter");
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
					again.set(true);
					on_search.call(client_query.clone().into());
				}
			>
				{button_name}
			</button>
		</div>
	}
}

