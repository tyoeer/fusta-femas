use leptos::*;
use leptos_router::{ActionForm, A};
use entities::*;
use crate::utils;
#[cfg(feature="ssr")]
use sea_orm::*;


#[server]
pub async fn new_feed(name: String, url: String, strategy: String) -> Result<i32, ServerFnError> {
	let conn = crate::extension!(DatabaseConnection);
	let mut new = feed::ActiveModel::new();
	new.name = Set(name);
	new.url = Set(url);
	//TODO validate
	new.strategy = Set(strategy);
	let inserted = new.insert(&conn).await?;
	Ok(inserted.id)
}

#[component]
pub fn FeedCreator() -> impl IntoView {
	let new_feed = create_server_action::<NewFeed>();
	view! {
		<ActionForm action=new_feed>
			<ul class="object_fieldvalue_list">
				<li class="object_fieldvalue">
					<label class="object_field" for="name_input"> name </label>
					<input class="object_value" type="text" name="name" id="name_input"/>
				</li>
				<li class="object_fieldvalue">
					<label class="object_field" for="url_input"> url </label>
					<input class="object_value" type="text" name="url" id="url_input"/>
				</li>
				<li class="object_fieldvalue">
					<label class="object_field" for="strategy_input"> strategy </label>
					<select class="object_value" name="strategy" id="strategy_input">
						<utils::AwaitOk future=crate::strategies::get_strategies let:strats>
							<For
								each=move || strats.clone()
								key=|s| s.clone()
								let:strat
							>
								<option value=strat.clone()> {strat} </option>
							</For>
						</utils::AwaitOk>
					</select>
				</li>
			</ul>
			
			<utils::FormSubmit button="create" action=new_feed/>
		</ActionForm>
		
		<utils::FormResult action=new_feed let:id>
			<A href=format!("/feed/{}", id)>"Created: " {id.to_string()}</A>
		</utils::FormResult>
	}
}

