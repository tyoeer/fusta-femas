use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
	use axum::{
		routing::get,
		response::{Response, IntoResponse},
		http::{Request, header::HeaderMap},
		extract::{Path, RawQuery, State},
		body::Body,
		Router,
	};
	use fusta_femas::app::*;
	use fusta_femas::fileserve::file_and_error_handler;
	use leptos::*;
	use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
	
	async fn leptos_server_fn_handler(
		path: Path<String>,
		headers: HeaderMap,
		raw_query: RawQuery,
		State(state): State<AppState>,
		req: Request<Body>
	) -> impl IntoResponse {
		handle_server_fns_with_context(
			path, headers, raw_query,
			move |cx| {
				provide_context(cx, state.conn.clone())
			},
			req
		).await
	}
	
	async fn leptos_route_handler(State(state): State<AppState>, req: Request<Body>) -> Response {
		let handler = leptos_axum::render_app_to_stream_with_context(
			state.leptos_options,
			move |cx| {
				provide_context(cx, state.conn.clone());
			},
			App
		);
		handler(req).await.into_response()
	}
	
	#[tokio::main]
	async fn main() {
		
		dotenvy::dotenv().ok();
		simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");
		
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
			leptos_options,
			conn,
		};
		
		// build our application with a route
		let app = Router::new()
			.route("/api/*fn_name", get(leptos_server_fn_handler).post(leptos_server_fn_handler))
			.leptos_routes_with_handler(generate_route_list(App).await, get(leptos_route_handler))
			.fallback(file_and_error_handler)
			.with_state(state)
		;

		// run our app with hyper
		// `axum::Server` is a re-export of `hyper::Server`
		log!("listening on http://{}", &addr);
		axum::Server::bind(&addr)
			.serve(app.into_make_service())
			.await
			.unwrap();
	}
} else {
	pub fn main() {
		// no client-side main function
		// unless we want this to work with e.g., Trunk for a purely client-side app
		// see lib.rs for hydration function instead
	}
}}
