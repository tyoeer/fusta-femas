use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
	#[tokio::main]
	async fn main() {
		let app = app::app::App;
		server_setup::run(app, |router| {
			backend::layer(router)
		}).await;
	}
} else {
	pub fn main() {
		// no client-side main function
		// unless we want this to work with e.g., Trunk for a purely client-side app
		// see lib.rs for hydration function instead
	}
}}
