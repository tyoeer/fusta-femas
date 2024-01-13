#[tokio::main]
async fn main() {	
	server_setup::run::<sea_migration::Migrator, _>(
		app::app::App,
		app::extend
	).await;
}