use leptos::*;
use leptos_router::{A, Params, IntoParam, use_query};
use entities::prelude::*;
use serde::{Serialize, Deserialize};
use crate::table::*;
use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;

#[derive(Debug,Clone,PartialEq,Eq, Params, Serialize,Deserialize)]
struct SearchParameters {
	tag: Option<i32>,
}

#[server]
pub async fn search(params: SearchParameters) -> Result<Vec<feed::Model>, ServerFnError> {
	let mut query = feed::Entity::find();
	if let Some(tag_id) = params.tag {
		query = query
			.inner_join(tag::Entity)
			.filter(tag::Column::Id.eq(tag_id));
	}
	let conn = crate::extension!(DatabaseConnection);
	let feeds = query.all(&conn).await?;
	Ok(feeds)
}

#[component]
pub fn Search() -> impl IntoView {
	
	let params_res_memo = use_query::<SearchParameters>();
	
	view! {
		<form method="get">
			<label for="tag">"tag"</label>
			<input type="number" name="tag" id="tag" />
			<input type="submit" value="search" />
		</form>
		
		{
			params_res_memo.get().map(|params| view!{
				<utils::AwaitOk future=move || search(params.clone()) let:feeds>
					<ObjectTable items = feeds />
				</utils::AwaitOk>
			})
		}
		
		<A href="new">Create new feed</A>
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