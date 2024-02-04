use leptos::*;
use leptos_router::{Params, use_query};
use entities::prelude::*;
use serde::{Serialize, Deserialize};
use crate::table::*;
use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;

#[derive(Debug,Clone,PartialEq,Eq, Params, Serialize,Deserialize)]
pub struct SearchParameters {
	tag: Option<tag::Ref>,
}

#[server]
pub async fn search(params: Option<SearchParameters>) -> Result<Vec<feed::Model>, ServerFnError> {
	let mut query = feed::Entity::find();
	//Params is an Option because it gets serialised into nothingness when empty
	//TODO figure out a better way of doing this
	if let Some(params) = params {
		if let Some(tag) = params.tag {
			query = tag.filter_related(query);
		}
	}
	let conn = crate::extension!(DatabaseConnection);
	let feeds = query.all(&conn).await?;
	Ok(feeds)
}

#[component]
pub fn Search() -> impl IntoView {
	
	let params_res_memo = use_query::<SearchParameters>();
	
	let enable_tag = RwSignal::new(false);
	
	view! {
		<form method="get">
			<label for="tag_enable">"tag"</label>
			<input type="checkbox" id="tag_enable" on:input=move |event| {
				enable_tag.set(event_target_checked(&event));
			}/>
			<select name="tag" id="tag" prop:disabled=move || !enable_tag.get()>
				<utils::AwaitOk future=crate::tag::search::all_tags let:tags>
					<For
						each=move || tags.clone()
						key=|tag| tag.id
						let:tag
					>
						<option value=tag.id> {tag.title} </option>
					</For>
				</utils::AwaitOk>
			</select>
			<input type="submit" value="search" />
		</form>
		
		{ move || {
			params_res_memo.get().map(|params| view!{
				<utils::AwaitOk future=move || search(Some(params.clone())) let:feeds>
					<ObjectTable items = feeds />
				</utils::AwaitOk>
			})
		} }	
	}
}


#[server]
pub async fn all_feeds() -> Result<Vec<feed::Model>, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	let feeds = feed::Entity::find().all(&conn).await?;
	Ok(feeds)
}

#[component]
pub fn All() -> impl IntoView {
	view! {
		<utils::AwaitOk future=all_feeds let:feeds>
			<ObjectTable items = feeds />
		</utils::AwaitOk>
	}
}