use leptos::*;
use leptos_router::{use_params, Params, IntoParam, ParamsError};

#[derive(leptos::Params, Clone, PartialEq, Eq)]
struct IdParam {
	// Has to be an Option due to Leptos limitations
	pub id: Option<i32>,
}

pub fn with_id_param<Mapped>(callback: impl Fn(i32) -> Mapped) -> Result<Mapped, ParamsError> {
	let id_res_memo = use_params::<IdParam>();
	let res = id_res_memo.get().map(|params| params.id);
	
	match res {
		Ok(Some(id)) => Ok(callback(id)),
		Ok(None) => Err(ParamsError::MissingParam("id".to_string())),
		Err(err) => Err(err),
	}
	
}

#[component]
pub fn AwaitOk<
	FutureOk:
		//Await requires the future output to be Serializable because it can run on the server
		serde::Serialize + serde::de::DeserializeOwned +
		Clone +
		//Wanted by await
		'static,
	FutureError:
		//Await requires the future output to be Serializable because it can run on the server
		serde::Serialize + serde::de::DeserializeOwned +
		//Required to be able to just render the error
		Into<leptos::error::Error> +
		Clone +
		//Wanted by await
		'static,
	Future:
		std::future::Future<Output = Result<FutureOk,FutureError>> +
		//Wanted by await
		'static,
	AsyncFunction:
		Fn() -> Future +
		//Wanted by await
		'static,
	ChildrenView:
		IntoView +
		//Not quite sure why this one is required
		'static,
	Children:
		Fn(FutureOk) -> ChildrenView +
		//Wanted by await
		'static,
>(
	future: AsyncFunction,
	children: Children,
) -> impl IntoView {
	//Not doing this causes a lifetime error I don't get, and this is what Await does, so it seems fine
	let children_stored = store_value(children);
	view! {
		<Await future let:result>
			{
				//The children of the ErrorBoundary become a closure, which would otherwise escape with result
				let result = result.clone();
				view! {
					<ErrorBoundary fallback = |errors| view!{ <crate::app::ErrorsView errors /> } >
						{
							//Removing the closure causes a lifetime error
							#[allow(clippy::redundant_closure)]
							children_stored.with_value(|children| result.map(children))
						}
					</ErrorBoundary>
				}
			}
		</Await>
	}
}

/**
Extracts an <axum::Extension> of the type given as argument.
Errors get propagated up with `?`.

# Example

```
let conn = extension!(DatabaseConnection);
```

*/
#[macro_export]
macro_rules! extension {
	($extension:ty) => {
		leptos_axum::extractor::<axum::Extension<$extension>>().await?.0
	};
}