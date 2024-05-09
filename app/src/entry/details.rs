use leptos::*;
use leptos_router::{A, Route, ActionForm};
use entities::prelude::*;
use crate::table;
use crate::fetch::search::FetchOverview;
use crate::utils;
use ff_object::ref_signal;
#[cfg(feature="ssr")]
use sea_orm::*;
#[cfg(feature="ssr")]
use ff_object::View;

#[component(transparent)]
pub fn Routes() -> impl IntoView {
	view! {
		<Route path="/:id" view=EntryContext>
			<utils::RouteAlias to="about" />
			<Route path="about" view = About />
			<Route path="embedded" view = Embed />
			<Route path="fetches" view = Fetches />
		</Route>
	}
}

#[component]
pub fn EntryContext() -> impl IntoView {
	view! {
		<utils::ObjectContext getter=get_entry>
			<Sidebar />
		</utils::ObjectContext>
	}
}


#[component]
pub fn Sidebar() -> impl IntoView {
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
pub async fn mark_viewed(entry: entry::Ref, viewed: bool) -> Result<entry::Model, ServerFnError> {
	let db = crate::extension!(DatabaseConnection);
	
	let mut entry_model = entry::ActiveModel::new();
	entry_model.viewed = Set(viewed);
	entry_model.id = Unchanged(entry.id());
	let entry = entry_model.update(&db).await?;
	
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
			<input type="hidden" name="entry" value=move || entry.get().id />
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
		<table::ObjectFieldValueList<EntryModel> object=entry overloads=vec![
			("feed_id", true, |entry| {
				//Grab id out because it otherwise will complain about fetch outliving the closure
				//Since the id is i32 which is Copy, it doesn't have that problem
				let id = entry.feed_id;
				view! {
					<A href=format!("/feed/{id}") class="object_fieldvalue">
						<span class="object_field"> feed_id </span>
						<span class="object_value"> {id} </span>
					</A>
				}.into_view()
			}),
			("view_url", true, |entry| {
				let url = entry.view_url.to_owned();
				view! {
					// <A> is always relative to the current route, and- uses an empty href if it tries to go to a different domain
					<a href=utils::format_link(url.clone()) class="object_fieldvalue">
						<span class="object_field"> view_url </span>
						<span class="object_value"> {url} </span>
					</a>
				}.into_view()
			}),
			("embed_url", true, |entry| {
				if let Some(url) = entry.embed_url.to_owned() {
					view! {
						// <A> is always relative to the current route, and- uses an empty href if it tries to go to a different domain
						<a href=utils::format_link(url.clone()) class="object_fieldvalue">
							<span class="object_field"> embed_url </span>
							<span class="object_value"> {url} </span>
						</a>
					}.into_view()
				} else {
					view! {
						<span class="object_fieldvalue">
							<span class="object_field"> embed_url </span>
							<span class="object_value">  </span>
						</span>
					}.into_view()
					
				}
			}),
		]/>
		<MarkViewedButton entry = entry/>
	}.into()
}

#[component]
pub fn Embed() -> impl IntoView {
	let entry = crate::model!(entry);
	let (maybe_embed_url, _) = slice!(entry.embed_url);
	let (view_url, _) = slice!(entry.view_url);
	
	let url = move || maybe_embed_url.get().unwrap_or_else(|| view_url.get());
	
	view! {
		<MarkViewedButton entry />
		<iframe class="grow" src=utils::format_link(url()) />
	}.into()	
}

#[server]
pub async fn get_fetches(entry: entry::Ref) -> Result<Vec<FetchOverview>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	
	let query = entry.find_related();
	FetchOverview::from_query(query)
		.all(&conn)
		.await
		.map_err(|e| e.into())	
}

#[component]
pub fn Fetches() -> impl IntoView {
	let entry = crate::model!(entry);
	let entry_ref = ref_signal(entry);
	
	view! {
		<utils::AwaitOk future=move || get_fetches(entry_ref.get()) let:fetches>
			<table::ObjectTable items = fetches />
		</utils::AwaitOk>
	}.into()
}