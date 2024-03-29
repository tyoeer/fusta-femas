use axum::{
	response::{IntoResponse, Response},
	http::{Uri, StatusCode, Request},
	extract::State,
	body::Body,
	Router, Extension,
};
use tokio::net::TcpListener;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use sea_orm_migration::MigratorTrait;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, EnvFilter, registry, prelude::*};



mod config;
pub mod setup;


const DEFAULT_LOG_FILTER: &str = "debug,hyper=info,sqlx=warn";


async fn get_static_file(uri: Uri, root: &str) -> Result<Response, (StatusCode, String)> {
	let req = Request::builder().uri(uri.clone()).body(Body::empty()).unwrap();
	// `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
	// This path is relative to the cargo root
	match ServeDir::new(root).oneshot(req).await {
		Ok(res) => Ok(res.map(Body::new)),
		Err(err) => Err((
			StatusCode::INTERNAL_SERVER_ERROR,
			format!("Something went wrong: {err}"),
		)),
	}
}

pub fn setup_leptos_routing<View: IntoView + 'static>(app: fn() -> View, leptos_options: LeptosOptions) -> Router {
	/*
		This can't be moved into it's own function because a function returning this would return
		impl Fn(State<AppState>, Uri, Request<Body>) -> impl Future<Output = AxumResponse>
		which has a return-position impl trait in a Fn trait, which isn't allowed yet:
		https://github.com/rust-lang/rust/issues/99697
	*/
	//Returns the file at the uri if it exists, otherwise renders the app
	let file_or_app_handler = move |
		State(state): State<LeptosOptions>,
		uri: Uri,
		req: Request<Body>
	| async move {
		let res = get_static_file(uri.clone(), &state.site_root).await.unwrap();

		if res.status() == axum::http::StatusCode::OK {
			res.into_response()
		} else {
			let handler = leptos_axum::render_app_to_stream(state, app);
			handler(req).await.into_response()
		}
	};
	
	// build our application with a route
	Router::new()
		.leptos_routes(&leptos_options, generate_route_list(app), app)
		// .fallback(file_and_error_handler)
		.fallback(file_or_app_handler)
		.with_state(leptos_options)
}

pub fn setup_logging() {
	let fmt_layer = fmt::layer()
		.event_format(fmt::format().pretty());
	let maybe_env_filter = EnvFilter::builder()
		.with_default_directive(LevelFilter::WARN.into())
		.try_from_env();
	let filter = maybe_env_filter.unwrap_or_else(|_|
		{
			EnvFilter::builder()
					.parse(DEFAULT_LOG_FILTER)
					.expect("hardcoded log filter should be correct")
		}
	);
	registry()
		.with(fmt_layer)
		.with(filter)
		.init();
}

fn setup_environment() {
	dotenvy::dotenv().ok();
	//We want backtraces for errors while fetching
	std::env::set_var("RUST_BACKTRACE", "1");
}

pub async fn run<Migrator: MigratorTrait, View>(app: fn() -> View, mut setup: setup::Setup) where
	View: IntoView + 'static
{
	setup_environment();
	//The log filter depends on the environment
	setup_logging();
	
	let setup_span = &tracing::info_span!("Server setup/startup");
	let setup_span_guard = setup_span.enter();
	
	tracing::info!(?setup);
	
	let settings = config::Settings::load();
	tracing::info!(?settings);
	
	let res = settings.ensure_folders_exist();
	match res {
		Ok(true) => {
			tracing::info!("Notice: created data storage (sub)folder(s)");
		}
		Ok(false) => (), // nothing happened, don't care
		Err(err) => {
			tracing::error!(?err, "Error ensuring data storage folders exists");
			panic!("{1}: {:?}", err, "ensuring data storage folders exist should work");
		}
	}
	
	let res = setup.saveload_strategy_configurations(&settings);
	if let Err(err) = res {
		tracing::error!(?err, "Error saving strategy configurations");
		panic!("{1}: {:?}", err, "saving strategy configurations should work");
	}
	
	let db_conn = sea_orm::Database::connect(settings.database_url).await.expect("failed connecting to db");
	//Keep migrations as a generic/function parameter to prevent recompilation whenever migrations change
	Migrator::up(&db_conn, None).await.expect("failed running database migrations");
	
	
	
	// A path of `None` means it uses environment values, see
	// https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain
	let leptos_config = get_configuration(None).await.unwrap();
	let leptos_options = leptos_config.leptos_options;
	let serve_address = leptos_options.site_addr;
	
	let router = setup_leptos_routing(app, leptos_options)
		.layer(Extension(db_conn));
	
	let router = setup.extend(router);
	
	drop(setup_span_guard);
	
	// run our app with hyper
	// `axum::Server` is a re-export of `hyper::Server`
	tracing::info!("listening on http://{}", &serve_address);
	
	let listener = TcpListener::bind(serve_address).await.unwrap();
	axum::serve(listener, router).await.unwrap();
}
