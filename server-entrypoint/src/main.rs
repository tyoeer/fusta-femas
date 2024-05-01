#[tokio::main]
async fn main() {
	let mut setup = server_setup::setup::Setup::default();
	
	setup.add_strategy(acquire::mock::MockStrat::default());
	setup.add_strategy(acquire::yt_dlp::YtDlpStrategy::default());
	
	setup.add_tag(ffilter::tags::feed_manual::FeedManual::default());
	setup.add_filter(ffilter::filters::Fetched::default());
	setup.add_filter(ffilter::filters::ArgTest::default());
	setup.add_filter(ffilter::filters::Tag::default());
	
	server_setup::run::<sea_migration::Migrator, _>(
		app::app::App,
		setup
	).await;
}