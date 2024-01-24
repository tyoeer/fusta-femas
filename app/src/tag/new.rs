use leptos::*;
use leptos_router::{ActionForm, A};
use entities::prelude::*;
use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;



#[server]
pub async fn new_tag(title: String, kind: String) -> Result<tag::Ref, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	let mut new = tag::ActiveModel::new();
	new.title = Set(title);
	//TODO validate
	new.kind = Set(kind);
	let inserted = new.insert(&conn).await?;
	Ok(inserted.id.into())
}

#[component]
pub fn TagCreator() -> impl IntoView {
	let new_tag = create_server_action::<NewTag>();
	view! {
		<ActionForm action=new_tag>
			<ul class="object_fieldvalue_list">
				<li class="object_fieldvalue">
					<label class="object_field" for="title_input"> name </label>
					<input class="object_value" type="text" name="title" id="title_input" size=50/>
				</li>
				<li class="object_fieldvalue">
					<label class="object_field" for="kind_input"> type </label>
					<select class="object_value" name="kind" id="kind_input">
						<For
							each=tag::types
							key=|s| s.clone()
							let:kind
						>
							<option value=kind.clone()> {kind} </option>
						</For>
					</select>
				</li>
			</ul>
			
			<utils::FormSubmit button="create" action=new_tag/>
		</ActionForm>
		
		<utils::FormResult action=new_tag let:tag_ref>
			<A href=format!("/tag/{}", tag_ref.id())>"Created: " {tag_ref.to_string()}</A>
		</utils::FormResult>
	}
}

