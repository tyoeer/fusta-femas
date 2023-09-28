use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
	use axum::{
		body::{boxed, Body, BoxBody},
		extract::State,
		response::IntoResponse,
		http::{Request, Response, StatusCode, Uri},
	};
	use axum::response::Response as AxumResponse;
	use tower::ServiceExt;
	use tower_http::services::ServeDir;
	use crate::app::*;
	
	pub async fn file_and_error_handler(uri: Uri, State(app_state): State<AppState>, req: Request<Body>) -> AxumResponse {
		let root = app_state.leptos_options.site_root.clone();
		let res = get_static_file(uri.clone(), &root).await.unwrap();

		if res.status() == StatusCode::OK {
			res.into_response()
		} else {
			let handler = leptos_axum::render_app_to_stream(app_state.leptos_options.to_owned(), App);
			handler(req).await.into_response()
		}
	}

	pub async fn get_static_file(uri: Uri, root: &str) -> Result<Response<BoxBody>, (StatusCode, String)> {
		let req = Request::builder().uri(uri.clone()).body(Body::empty()).unwrap();
		// `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
		// This path is relative to the cargo root
		match ServeDir::new(root).oneshot(req).await {
			Ok(res) => Ok(res.map(boxed)),
			Err(err) => Err((
				StatusCode::INTERNAL_SERVER_ERROR,
				format!("Something went wrong: {err}"),
			)),
		}
	}
}}
