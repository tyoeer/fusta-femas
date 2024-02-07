use axum::{
	routing::Router,
	Extension,
};
use acquire::{
	batch_tracker::BatchTracker,
	strategy::Strategy,
	StrategyList,
};
use ffilter::{
	tag::Tag,
	tag_list::TagList,
};
use std::{fs::File, io::{Error as IoError, Write}};

use super::config::Settings;


const STRATEGY_CONFIG_FILE_EXTENSION: &str = "ron";


pub fn strategy_serializer<Writer: Write>(writer: Writer) -> ron::Result<ron::Serializer<Writer>> {
	use ron::*;
	Serializer::new(
		writer,
		Some(ser::PrettyConfig::new()
			.indentor("\t".into())
			.struct_names(true)
		),
	)
}

//TODO just use anyhow at this point
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum StrategySaveLoadError {
	Ron(#[from] ron::Error),
	RonSpanned(#[from] ron::error::SpannedError),
	Io(#[from] IoError),
	Serde(#[from] erased_serde::Error)
}

/**
Hard-coded configuration stuff:
- Fetch strategies
*/
#[derive(Default)]
pub struct Setup {
	pub strategies: Vec<Box<dyn Strategy + Send + Sync>>,
	pub tags: Vec<Box<dyn Tag + Send + Sync>>,
}

impl Setup {
	pub fn add_strategy(&mut self, strategy: impl Strategy + Send + 'static) {
		self.strategies.push(Box::new(strategy));
	}
	pub fn add_tag(&mut self, tag: impl Tag + Send + Sync + 'static) {
		self.tags.push(Box::new(tag));
	}
	
	pub fn saveload_strategy_configurations(&mut self, settings: &Settings) -> Result<(), StrategySaveLoadError> {
		let base_path = settings.get_strategy_config_path();
		
		for strat in &mut self.strategies {
			let mut path = base_path.join(strat.name());
			path.set_extension(STRATEGY_CONFIG_FILE_EXTENSION);
			if path.try_exists()? {
				let text = std::fs::read_to_string(path)?;
				let mut deserializer = ron::Deserializer::from_str(&text)?;
				let mut erased = <dyn erased_serde::Deserializer>::erase(&mut deserializer);
				strat.deserialize_replace(&mut erased)?;
			} else {
				let file = File::create(path)?;
				let mut serializer = strategy_serializer(file)?;
				let mut erased = <dyn erased_serde::Serializer>::erase(&mut serializer);
				strat.serialize(&mut erased)?;
			}
		}
		
		Ok(())
	}
	
	pub fn extend(self, router: Router) -> Router {
		let mut strat_list = StrategyList::new();
		for strat in self.strategies {
			strat_list.add_from_container(strat);
		}
		
		let mut tag_list = TagList::new();
		for tag in self.tags {
			tag_list.add_from_container(tag);
		}
		
		router
			.layer(Extension(tag_list))
			.layer(Extension(strat_list))
			.layer(Extension(BatchTracker::default()))
	}
	
	pub fn extend_fn(self) -> impl FnOnce(Router) -> Router {
		move |router| self.extend(router)
	}
}

impl std::fmt::Debug for Setup {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let strategy_names = self.strategies.iter()
			.map(|s| s.name())
			.collect::<Vec<_>>();
		f.debug_struct("Setup")
			.field("strategy_names", &strategy_names)
			.finish()
	}
}