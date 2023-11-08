use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
	mod setup_server;

	#[tokio::main]
	async fn main() {
		let app = app::app::App;
		setup_server::run(app).await;
	}
} else {
	pub fn main() {
		// no client-side main function
		// unless we want this to work with e.g., Trunk for a purely client-side app
		// see lib.rs for hydration function instead
	}
}}
