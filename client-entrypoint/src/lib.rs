use wasm_bindgen::prelude::wasm_bindgen;
use tracing_subscriber::{*, prelude::*};

#[wasm_bindgen]
pub fn hydrate() {
	// initializes logging using the `log` crate
	let fmt_layer = fmt::layer()
		.with_ansi(false)
		.with_timer(fmt::time::UtcTime::rfc_3339())
		.with_writer(tracing_web::MakeConsoleWriter);
	registry()
		.with(fmt_layer)
		.init();
	
	console_error_panic_hook::set_once();

	leptos::mount_to_body(app::app::App);
}