use leptos::*;
use serde::{Deserialize, Serialize};


pub mod filter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
	filter: filter::Filter,
}

impl Query {
	pub fn from_filter_name(name: impl Into<String>) -> Self {
		Self {
			filter: filter::Filter::from_name(name.into()),
		}
	}
	
	pub fn filter_signal(self_signal: RwSignal<Self>) -> RwSignal<filter::Filter> {
		
		let filter_signal = RwSignal::new(self_signal.with(|q| q.filter.clone()));
		
		//sync
		//we aren't supposed to do this, but we would need https://github.com/leptos-rs/leptos/issues/2358
		create_effect(move |old| {
			let filter = filter_signal.get();
			
			let changed = match old {
				None => true,
				Some(old_filter) => old_filter != filter,
			};
			
			if changed {
				self_signal.update(|q| q.filter = filter.clone());
			}
			
			filter
		});
		create_effect(move |old| {
			let filter = self_signal.get().filter;
			
			let changed = match old {
				None => true,
				Some(old_filter) => old_filter != filter,
			};
			
			if changed {
				filter_signal.update(|q| {
					let _ = std::mem::replace(q, filter.clone());
				});
			}
			
			filter
		});
		
		filter_signal
	}
}


#[component]
pub fn QueryUI<ActionOutput: 'static>(action: Action<Query, Result<ActionOutput, ServerFnError>>) -> impl IntoView {
	let query_signal = RwSignal::new(Query::from_filter_name(""));
	
	let filter_signal = Query::filter_signal(query_signal);
	
	let button_name = move || {
		if action.pending().get() {
			"searching...".to_owned()
		} else {
			format!("search{}", if action.value().with(|val| val.is_some()) {" again"} else {""} )
		}
	};
	
	view! {
		<filter::Filter filter=filter_signal sub_id=""/>
		
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

