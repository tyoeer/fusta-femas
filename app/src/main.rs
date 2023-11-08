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
	use app::app::*;
	use app::fileserve::get_static_file;
	use leptos::*;
	use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
	use tracing_subscriber::{*, prelude::*};
	
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
	
	#[tokio::main]
	async fn main() {
		
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
		let leptos_route_handler = {
			let app = App;
			
			move |State(state): State<AppState>, req: Request<Body>| async move {
				let handler = leptos_axum::render_app_to_stream_with_context(
					state.leptos_options,
					move |cx| {
						provide_context(cx, state.conn.clone());
					},
					app
				);
				handler(req).await.into_response()
			}
		};
		//Returns the file at the uri if it exists, otherwise renders the app
		let file_or_app_handler = {
			//Explicitly mention it here because it gets captured by the closure,
			// and to potentially add a .clone() later if it gets necessary
			let leptos_options = leptos_options;
			let app = App;
			
			move |uri: axum::http::Uri, req| async move {
				let res = get_static_file(uri.clone(), &leptos_options.site_root).await.unwrap();

				if res.status() == axum::http::StatusCode::OK {
					res.into_response()
				} else {
					let handler = leptos_axum::render_app_to_stream(leptos_options, app);
					handler(req).await.into_response()
				}
			}
		};
		
		// build our application with a route
		let app = Router::new()
			.route("/api/*fn_name", get(leptos_server_fn_handler).post(leptos_server_fn_handler))
			.leptos_routes_with_handler(generate_route_list(App).await, get(leptos_route_handler))
			// .fallback(file_and_error_handler)
			.fallback(file_or_app_handler)
			.with_state(state)
		;
		let app = backend::layer(app);
		
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
