use leptos::*;
use serde::{Deserialize, Serialize};


pub mod filter;
use filter::ClientFilter;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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


#[component]
pub fn QueryUI<ActionOutput: 'static>(action: Action<Query, Result<ActionOutput, ServerFnError>>) -> impl IntoView {
	let button_name = move || {
		if action.pending().get() {
			"searching...".to_owned()
		} else {
			format!("search{}", if action.value().with(|val| val.is_some()) {" again"} else {""} )
		}
	};
	
	let filter: RwSignal<Option<RwSignal<ClientFilter>>> = RwSignal::new(None);
	
	let filter_ui = move || {
		match filter.get() {
			Some(filter_sig) => {
				let (get, set) = filter_sig.split();
				view! {
					<filter::Filter get=get.into() set=set.into() sub_id="" />
				}.into_view()
			},
			None => ().into_view(),
		}
	};
	
	view! {
		<div class="search">
			<div class="search_parameters">
				<label for="filter_enable">filter</label>
				<input type="checkbox" id="filter_enable" on:input=move |event| {
					if event_target_checked(&event) {
						filter.set(Some(RwSignal::new(ClientFilter::from_name(""))));
					} else {
						filter.set(None);
					}
				}/>
				{filter_ui}
			</div>
			
			<button
				disabled=move || action.pending().get()
				on:click = move |_event| {
					action.dispatch(Query::from_filter_signal(filter));
				}
			>
				{button_name}
			</button>
		</div>
	}
}

