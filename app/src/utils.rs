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
Form submit button that disables itself while the server action is running.
Also updates the button text.
*/
#[component]
pub fn FormSubmit<
	ActionInput: 'static,
	ActionOutput: 'static,
>(
	#[prop(into)]
	button: String,
	action: Action<ActionInput, Result<ActionOutput, ServerFnError>>
) -> impl IntoView {
	let button_name = move || {
		if action.pending().get() {
			format!("{button}ing...")
		} else {
			format!("{button}{}", if action.value().with(|val| val.is_some()) {" again"} else {""} )
		}
	};
	view! {
		<input type="submit" value=button_name disabled=move || action.pending().get() />
	}
}

/**
Unpackages the result of a server action.

Displays:
- If `action` is yet to be run: nothing
- If `action` returned an error: the error, and logs it with some extra info
- If `action` was successful: render `children` with the action output
*/
#[component]
pub fn FormResult<
	ActionInput: Clone + std::fmt::Debug + 'static,
	ActionOutput: Clone + 'static,
	Children: Fn(ActionOutput) -> ChildrenView + 'static,
	ChildrenView: IntoView,
>(
	#[prop(into)]
	action: Action<ActionInput, Result<ActionOutput, ServerFnError>>,
	children: Children,
) -> impl IntoView {
	move || {
		match action.value().get() {
			Some(Ok(output)) => {
				children(output).into_view()
			},
			Some(Err(error)) => {
				tracing::error!(input = ?action.input().get(), error=error.to_string(), url=action.url(), "Error occurred submitting form to server:");
				format!("Server error: {error}").into_view()
			},
			None => ().into_view()
		}
	}
}

/**
Extracts an <axum::Extension> of the type given as argument.
Errors get propagated up with `?`.

# Example

```no_run
# use app::extension;
# use sea_orm::DatabaseConnection;
# use leptos::ServerFnError;
# async fn server_fn() -> Result<(), ServerFnError> {
let conn = extension!(DatabaseConnection);
# Ok(())
# };
```

*/
#[macro_export]
macro_rules! extension {
	($extension:ty) => {
		leptos_axum::extractor::<axum::Extension<$extension>>().await?.0
	};
}

/**
Grabs a model from the entities crate from the Leptos context, and puts it into scope.

# Example

```no_run
# use app::model;
# use leptos::use_context;
# use entities::entry;
model!(entry);
dbg!(entry);
```

*/
#[macro_export]
macro_rules! model {
	($entity:ident) => {
		{
			let Some(model) = ::leptos::use_context::<::leptos::RwSignal<$entity::Model>>() else {
				return None;
			};
			model
		}
	};
}