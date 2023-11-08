use axum::{
	routing::get,
	response::{IntoResponse, Response},
	http::{Uri, StatusCode, Request, header::HeaderMap},
	extract::{Path, RawQuery, State},
	body::{boxed, Body},
	Router,
};
use leptos::*;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
use tower::ServiceExt;
use tower_http::services::ServeDir;
use tracing_subscriber::{*, prelude::*};

use app::app::AppState;

async fn get_static_file(uri: Uri, root: &str) -> Result<Response, (StatusCode, String)> {
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

async fn leptos_server_fn_handler(
	path: Path<String>,
	headers: HeaderMap,
	raw_query: RawQuery,
	State(state): State<AppState>,
	req: Request<Body>
) -> impl IntoResponse {
	handle_server_fns_with_context(
		path, headers, raw_query,
		move || {
			provide_context(state.conn.clone())
		},
		req
	).await
}

pub async fn run<View>(app: fn() -> View) where
	View: IntoView + 'static
{
	// let app = app::app::App;
	
	dotenvy::dotenv().ok();
	
	let fmt_layer = fmt::layer()
		.event_format(fmt::format().pretty());
	registry()
		.with(fmt_layer)
		.with(EnvFilter::from_default_env())
		.init();
	// Setting get_configuration(None) means we'll be using cargo-leptos's env values
	// For deployment these variables are:
	// <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
	// Alternately a file can be specified such as Some("Cargo.toml")
	// The file would need to be included with the executable when moved to deployment
	let conf = get_configuration(None).await.unwrap();
	let leptos_options = conf.leptos_options;
	let addr = leptos_options.site_addr;
	
	let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
	let conn = sea_orm::Database::connect(db_url).await.expect("failed connecting to db");
	
	let state = AppState {
		leptos_options: leptos_options.clone(),
		conn,
	};
	
	/*
		These can't be moved into their own module because a function returning one of these would return
		impl Fn(Uri, State<AppState>, Request<Body>) -> impl Future<Output = AxumResponse>
		which has a return-position impl trait in a Fn trait, which isn't allowed yet:
		https://github.com/rust-lang/rust/issues/99697
	*/
	//Renders the leptos app
	let leptos_route_handler = move |State(state): State<AppState>, req: Request<Body>| async move {
		let handler = leptos_axum::render_app_to_stream_with_context(
			state.leptos_options,
			move || {
				provide_context(state.conn.clone());
			},
			app
		);
		handler(req).await.into_response()
	};
	//Returns the file at the uri if it exists, otherwise renders the app
	let file_or_app_handler = move |State(state): State<AppState>, uri: axum::http::Uri, req| async move {
		let res = get_static_file(uri.clone(), &state.leptos_options.site_root).await.unwrap();

		if res.status() == axum::http::StatusCode::OK {
			res.into_response()
		} else {
			let handler = leptos_axum::render_app_to_stream(state.leptos_options, app);
			handler(req).await.into_response()
		}
	};
	
	// build our application with a route
	let app = Router::new()
		.route("/api/*fn_name", get(leptos_server_fn_handler).post(leptos_server_fn_handler))
		.leptos_routes_with_handler(generate_route_list(app), get(leptos_route_handler))
		// .fallback(file_and_error_handler)
		.fallback(file_or_app_handler)
		.with_state(state)
	;
	let app = backend::layer(app);
	
	// run our app with hyper
	// `axum::Server` is a re-export of `hyper::Server`
	tracing::info!("listening on http://{}", &addr);
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
