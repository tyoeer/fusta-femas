use leptos::*;
use leptos_router::{Route, A, ActionForm};
use entities::prelude::*;
use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;


#[server]
pub async fn fetch_all() -> Result<(), ServerFnError> {
	let db = crate::extension!(DatabaseConnection);
	let strats = crate::extension!(acquire::StrategyList);
	
	let feeds = feed::Entity::find().all(&db).await?;
	
	let feeds = feeds.into_iter().map(|feed| feed.id).collect();
	
	tokio::spawn(acquire::batch::fetch_batch(feeds, strats, db));
	
	Ok(())
}

#[component]
pub fn FetchAllButton() -> impl IntoView {
	let fetch_all = create_server_action::<FetchAll>();
	view! {
		<ActionForm action=fetch_all>
			<utils::FormSubmit button="fetch all feeds" action=fetch_all/>
		</ActionForm>
		<utils::FormResult action=fetch_all let:_>
			"Started fetch"
		</utils::FormResult>
	}
}