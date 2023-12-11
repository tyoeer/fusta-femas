use leptos::*;
use crate::utils;

#[server]
pub async fn get_strategies() -> Result<Vec<String>, ServerFnError> {	
	let strats = crate::extension!(acquire::strategy_list::StrategyList);
	let list = strats.iter_strats().map(|s| s.name().to_owned()).collect::<Vec<String>>();
	Ok(list)
}

#[component]
pub fn Strategies() -> impl IntoView {
	view! {
		<utils::AwaitOk future=get_strategies let:strats>
			<ul>
				{
					strats.into_iter()
						.map(|e| view! {<li>{e}</li>})
						.collect::<Vec<_>>()
				}
			</ul>
		</utils::AwaitOk>
	}
}