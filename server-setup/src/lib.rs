use axum::{
	routing::get,
	response::{IntoResponse, Response},
	http::{Uri, StatusCode, Request},
	extract::State,
	body::{boxed, Body},
	Router, Extension, Server,
};
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns};
use sea_orm_migration::MigratorTrait;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, EnvFilter, registry, prelude::*};

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


pub async fn run<Migrator: MigratorTrait, View>(app: fn() -> View, extend: impl FnOnce(Router) -> Router) where
	View: IntoView + 'static
{
	//We want backtraces for errors while fetching
	std::env::set_var("RUST_BACKTRACE", "1");
	
	dotenvy::dotenv().ok();
	
	let fmt_layer = fmt::layer()
		.event_format(fmt::format().pretty());
	let maybe_env_filter = EnvFilter::builder()
		.with_default_directive(LevelFilter::WARN.into())
		.try_from_env();
	let filter = maybe_env_filter.unwrap_or_else(|_|
		EnvFilter::builder()
			.parse("debug,hyper=info,sqlx=warn")
			.expect("hardcoded log filter should be correct")
	);
	registry()
		.with(fmt_layer)
		.with(filter)
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
	
	//Keep migrations as a generic/function parameter to prevent recompilation whenever migrations change
	Migrator::up(&conn, None).await.expect("failed running database migrations");
	
	/*
		This can't be moved into it's own function because a function returning this would return
		impl Fn(State<AppState>, Uri, Request<Body>) -> impl Future<Output = AxumResponse>
		which has a return-position impl trait in a Fn trait, which isn't allowed yet:
		https://github.com/rust-lang/rust/issues/99697
	*/
	//Returns the file at the uri if it exists, otherwise renders the app
	let file_or_app_handler = move |State(state): State<LeptosOptions>, uri: Uri, req: Request<Body>| async move {
		let res = get_static_file(uri.clone(), &state.site_root).await.unwrap();

		if res.status() == axum::http::StatusCode::OK {
			res.into_response()
		} else {
			let handler = leptos_axum::render_app_to_stream(state, app);
			handler(req).await.into_response()
		}
	};
	
	// build our application with a route
	let app = Router::new()
		.route("/api/*fn_name", get(handle_server_fns).post(handle_server_fns))
		.leptos_routes(&leptos_options, generate_route_list(app), app)
		// .fallback(file_and_error_handler)
		.fallback(file_or_app_handler)
		.with_state(leptos_options)
		.layer(Extension(conn))
	;
	
	let app = extend(app);
	
	// run our app with hyper
	// `axum::Server` is a re-export of `hyper::Server`
	tracing::info!("listening on http://{}", &addr);
	Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
