use leptos::*;
use leptos_router::ActionForm;
use entities::*;
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
	let button_name = move || {
		if new_feed.pending().get() {
			"creating...".to_owned()
		} else {
			match new_feed.value().get() {
				None => {
					"Create".to_owned()
				},
				Some(res) => match res {
					Ok(id) => format!("created : {id}"),
					Err(err) => format!("server error: {err}"),
				}
			}
		}
	};
	view! {
		<ActionForm action=new_feed>
			<input type="text" name="name" />
			<input type="text" name="url" />
			<select name="strategy">
				<Await future=crate::strategies::get_strategies let:strats>
					{
						strats.clone().map(|strats| {
							view! {
								<For
									each=move || strats.clone()
									key=|s| s.clone()
									let:strat
								>
									<option value=strat.clone()> {strat} </option>
								</For>
							}
						})
					}
					
				</Await>
			</select>
			<input type="submit" value=button_name/>
		</ActionForm>
	}
}

