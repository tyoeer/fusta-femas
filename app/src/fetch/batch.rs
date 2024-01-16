use std::time::Duration;

use leptos::*;
use leptos_router::{Route, ActionForm, Outlet, Redirect, A};
#[cfg(feature="ssr")]
use entities::prelude::*;
use serde::{Serialize, Deserialize};
use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;
#[cfg(feature="ssr")]
use acquire::batch::Batch;

#[component(transparent)]
pub fn Routes() -> impl IntoView {
	view! {
		<Route path="fetch_batch" view=Outlet>
			<Route path="/:id" view=UpdatingBatchPage />
		</Route>
	}
}

///Transportable batch status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchStatus {
	total: usize,
	done: usize,
	id: usize,
}

impl BatchStatus {
	#[cfg(feature="ssr")]
	pub fn from_id_batch(id: usize, batch: &Batch) -> Self {
		Self {
			total: batch.total,
			done: batch.finished.len(),
			id,
		}
	}
	
	pub fn is_finished(&self) -> bool {
		self.total == self.done
	}
}


#[server]
pub async fn get_batch_status(batch_ref: usize) -> Result<BatchStatus, ServerFnError> {
	let tracker = crate::extension!(acquire::batch_tracker::BatchTracker);
	
	let batch_sync = tracker.get_status(batch_ref).await?;
	let status = { // Scope to reduce lock time
		let batch_lock = batch_sync.read().await;
		BatchStatus::from_id_batch(batch_ref, &batch_lock)
	};
	
	Ok(status)
}


const REFRESH_INTERVAL: Duration = Duration::from_millis(1000);

#[component(transparent)]
pub fn UpdatingBatchPage() -> impl IntoView {
	utils::react_id(move |id| {
		let resource = create_resource(
			|| (),
			move |_| get_batch_status(id as usize)
		);
		let handle_store = RwSignal::<Option<leptos_dom::helpers::TimeoutHandle>>::new(None);
		//Repeatedly refetch status
		create_effect(move |_| {
			if let Some(handle) = handle_store.get() {
				handle.clear();
			}
			if let Some(Ok(batch_status)) = resource.get() {
				if !batch_status.is_finished() {
					//Use timeout instead of interval to wait until it has updated and not overfetch the server
					let handle_res = set_timeout_with_handle(move || resource.refetch(), REFRESH_INTERVAL);
					match handle_res {
						Ok(handle) => handle_store.set(Some(handle)),
						Err(err) => {
							tracing::error!(?err, "Error setting timeout");
						}
					}
				}
			}
		});
		on_cleanup(move || {
			if let Some(handle) = handle_store.get() {
				handle.clear();
			}
		});
		
		
		view! {
			<leptos_meta::Title text=format!("batch {}", id) />
			<main>
				<utils::ResourceOk
					fallback = || view! {<div>"Loading..."</div>}
					resource
					let:batch
				>
					<BatchInfo batch/>
				</utils::ResourceOk>
			</main>
		}
	})
}

#[component]
pub fn BatchInfo(#[prop(into)] batch: MaybeSignal<BatchStatus>) -> impl IntoView {
	view! {
		<div> {format!("Finished: {} / {}",
			batch.get().done,
			batch.get().total,
		)} </div>
		<progress max=batch.get().total value=batch.get().done/>
	}
}



#[server]
pub async fn fetch_all() -> Result<usize, ServerFnError> {
	let db = crate::extension!(DatabaseConnection);
	let strats = crate::extension!(acquire::StrategyList);
	let tracker = crate::extension!(acquire::batch_tracker::BatchTracker);
	
	let feeds = feed::Entity::find().all(&db).await?;
	
	let feeds = feeds.into_iter().map(|feed| feed.id).collect();
	
	let batch_id = tracker.queue_fetches(feeds, db, strats).await;
	
	Ok(batch_id)
}

#[component]
pub fn FetchAllButton(#[prop(default=false)] redirect: bool) -> impl IntoView {
	let fetch_all = create_server_action::<FetchAll>();
	let text = if redirect {"fetch all feeds"} else {"fetch all feeds in bg"};
	view! {
		<ActionForm action=fetch_all>
			<utils::FormSubmit button=text action=fetch_all/>
		</ActionForm>
		<utils::FormResult action=fetch_all let:id>
			{
				let url = format!("/fetch_batch/{id}");
				if redirect {
					view! { <Redirect path=url /> }
				} else {
					view! { <A href=url>{ format!("Started fetch with id {id}") }</A> }
				}
			}
		</utils::FormResult>
	}
}