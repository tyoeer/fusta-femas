use leptos::*;
use leptos_router::{A, Outlet, Route, Redirect, ActionForm};
use entities::prelude::*;
use crate::table;
use crate::fetch::search::FetchOverview;
use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;

#[component(transparent)]
pub fn Routes() -> impl IntoView {
	view! {
		<Route path="/:id" view=ObjectContext>
			<Route path="" view=|| view! { <Redirect path="about"/> }/>
			<Route path="about" view = About />
			<Route path="embedded" view = Embed />
			<Route path="fetches" view = Fetches />
		</Route>
	}
}

#[component]
pub fn ObjectContext() -> impl IntoView {
	utils::react_id(|id| view! {
		<Navbar />
		<main>
			<utils::AwaitOk future=move || get_entry(id) let:entry>
				{
					provide_context(create_rw_signal(entry));
					
					view! {
						<Outlet/>
					}
				}
			</utils::AwaitOk>
		</main>
	})
}

#[component]
pub fn Navbar() -> impl IntoView {
	view! {
		<nav class="sidebar">
			<ul>
				<li>
					<A href="about">About</A>
				</li>
				<li>
					<A href="embedded">Embedded</A>
				</li>
				<li>
					<A href="fetches">Fetches</A>
				</li>
			</ul>
		</nav>
	}
}


#[server]
pub async fn get_entry(id: i32) -> Result<entry::Model, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	entry::Entity::find_by_id(id)
		.one(&conn)
		.await?
		.ok_or(
			ServerFnError::ServerError("No such feed".into())
		)
}

#[server]
pub async fn mark_viewed(id: i32, viewed: bool) -> Result<entry::Model, ServerFnError> {
	let db = crate::extension!(DatabaseConnection);
	
	let mut entry = entry::ActiveModel::new();
	entry.viewed = Set(viewed);
	entry.id = Unchanged(id);
	let entry = entry.update(&db).await?;
	
	Ok(entry)
}

#[component]
pub fn MarkViewedButton(entry: RwSignal<entry::Model>) -> impl IntoView {
	let action = create_server_action::<MarkViewed>();
	let viewed = move || entry.get().viewed;
	let un = move || if viewed() {"un"} else {""};
	
	//TODO the book says effects shouldn't be used for this
	create_isomorphic_effect(move |_| {
		if let Some(Ok(new_entry)) = action.value().get() {
			entry.set(new_entry);
		}
	});
	
	let button_name = move || {
		if action.pending().get() {
			format!("marking as {}viewed...", un() )
		} else {
			format!("mark {}viewed", un() )
		}
	};
	
	view! {
		<ActionForm action = action>
			<input type="hidden" name="id" value=move || entry.get().id />
			<input type="hidden" name="viewed" value=move || (!viewed()).to_string() />
			<input type="submit" value=button_name disabled=move || action.pending().get() />
		</ActionForm>
	}
}

#[component]
pub fn About() -> impl IntoView {
	let entry = crate::model!(entry);
	
	//Need to use a use statement because the view! macro can't parse a :: in a generic
	use entry::Model as EntryModel;
	
	view! {
		//Need to manually specify generic because it can't infer type because we might want to put a MaybeSignal in a MaybeSignal
		<table::ObjectFieldValueList<EntryModel> object=entry />
		<MarkViewedButton entry = entry/>
	}.into()
}

#[component]
pub fn Embed() -> impl IntoView {
	let entry = crate::model!(entry);
	let (maybe_url, _) = slice!(entry.embed_url);
	
	Some( move || match maybe_url.get() {
		Some(mut url) => {
			if !url.contains("://") {
				url = format!("https://{url}");
			}
			view! {
				<iframe class="grow" src=url />
			}.into_view()
		},
		None => {
			view! {
				"Entry has no embed url specified 🤷"
			}.into_view()
		},
	} )
}

#[server]
pub async fn get_fetches(entry_id: i32) -> Result<Vec<FetchOverview>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	let query = <entry::Entity as Related<fetch::Entity>>::find_related()
		.filter(entry::Column::Id.eq(entry_id));
	FetchOverview::from_query(query)
		.all(&conn)
		.await
		.map_err(|e| e.into())	
}

#[component]
pub fn Fetches() -> impl IntoView {
	let entry = crate::model!(entry);
	let (id, _) = slice!(entry.id);
	
	view! {
		<utils::AwaitOk future=move || get_fetches(id.get()) let:fetches>
			<table::ObjectTable items = fetches />
		</utils::AwaitOk>
	}.into()
}