///A way to fetch entries for a single feed
pub mod strategy;
pub mod strategy_list;
pub mod yt_dlp;
pub mod mock;
///System for fetching a list of feeds
pub mod batch;

pub use strategy_list::StrategyList;
pub use strategy_list::RunError;