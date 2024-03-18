use leptos::*;
use serde::{Deserialize, Serialize};


pub mod filter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
	filter: filter::ClientFilter,
}

impl Query {
	pub fn from_filter_name(name: impl Into<String>) -> Self {
		Self {
			filter: filter::ClientFilter::from_name(name.into()),
		}
	}
	
	pub fn into_filter(self) -> filter::ClientFilter {
		self.filter
	}
	
	pub fn filter_signal(self_signal: RwSignal<Self>) -> (Signal<filter::ClientFilter>, SignalSetter<filter::ClientFilter>) {
		slice!(self_signal.filter)
	}
}


#[component]
pub fn QueryUI<ActionOutput: 'static>(action: Action<Query, Result<ActionOutput, ServerFnError>>) -> impl IntoView {
	let query_signal = RwSignal::new(Query::from_filter_name(""));
	
	let (get, set) = Query::filter_signal(query_signal);
	
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
		<input type="checkbox" id="tag_enable" on:input=move |event| {
			if event_target_checked(&event) {
				filter.set(Some(RwSignal::new(ClientFilter::from_name(""))));
			} else {
				filter.set(None);
			}
		}/>
		{filter_ui}
		
		<button
			disabled=move || action.pending().get()
			on:click = move |event| {
				event.prevent_default();
				action.dispatch(query_signal.get());
			}
		>
			{button_name}
		</button>
	}
}

