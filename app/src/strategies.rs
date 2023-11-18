use leptos::*;

#[cfg(feature="ssr")]
pub async fn get_strats() -> Result<acquire::strategy_list::StrategyList, ServerFnError> {
	use acquire::strategy_list::StrategyList;
	use axum::*;
	use leptos_axum::extractor;
	
	extractor::<Extension<StrategyList>>().await.map(|ext| ext.0)
}

#[server]
pub async fn get_strategies() -> Result<Vec<String>, ServerFnError> {	
	let strats = get_strats().await?;
	let list = strats.iter_strats().map(|s| s.name().to_owned()).collect::<Vec<String>>();
	Ok(list)
}

#[component]
pub fn Strategies() -> impl IntoView {
	view! {
		<Await future=get_strategies let:strats>
			<ul>
				{
					strats.clone().map(|vec| {
						vec.into_iter()
							.map(|e| view! {<li>{e}</li>})
							.collect::<Vec<_>>()
					})
				}
			</ul>
		</Await>
	}
}