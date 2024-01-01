use leptos::*;
use leptos_router::{Route, ActionForm, Outlet, Redirect, A};
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
			<Route path="/:id" view=BatchContext>
				<Route path="" view=BatchInfo />
			</Route>
		</Route>
	}
}

#[component]
pub fn BatchContext() -> impl IntoView {
	let getter = |id| get_batch_status(id as usize);
	view! {
		<utils::ObjectContext getter>
			""
		</utils::ObjectContext>
	}
}

#[component]
pub fn BatchInfo() -> impl IntoView {
	let object = crate::object!(BatchStatus);
	
	Some(move || format!("{:?}", object.get()))
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
}

impl Object for BatchStatus {
	fn get_id(&self) -> i32 {
		self.id as i32
	}
	
	fn get_object_name() -> &'static str {
		"batch_fetch"
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