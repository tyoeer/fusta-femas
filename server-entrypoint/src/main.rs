#[tokio::main]
async fn main() {
	server_setup::run(
		app::app::App,
		app::extend
	).await;
}