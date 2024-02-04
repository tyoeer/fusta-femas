#[tokio::main]
async fn main() {
	let mut setup = server_setup::setup::Setup::default();
	
	setup.add_strategy(acquire::mock::MockStrat::default());
	setup.add_strategy(acquire::yt_dlp::YtDlpStrategy::default());
	
	setup.add_tag(tags::tags::feed_manual::FeedManual::default());
	
	server_setup::run::<sea_migration::Migrator, _>(
		app::app::App,
		setup
	).await;
}