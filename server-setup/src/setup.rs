use axum::{
	routing::Router,
	Extension,
};
use acquire::{
	batch_tracker::BatchTracker,
	strategy::Strategy,
	StrategyList,
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

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum StrategyIError {
	Ron(#[from] ron::Error),
	Io(#[from] IoError),
	Serde(#[from] erased_serde::Error)
}

#[derive(Default)]
pub struct Setup {
	pub strategies: Vec<Box<dyn Strategy + Send + Sync>>,
}

impl Setup {
	pub fn add_strategy(&mut self, strategy: impl Strategy + Send + 'static) {
		self.strategies.push(Box::new(strategy));
	}
	
	pub fn save_strategy_configurations(&self, settings: &Settings) -> Result<(), StrategyIError> {
		let base_path = settings.get_strategy_config_path();
		
		for strat in &self.strategies {
			let mut path = base_path.join(strat.name());
			path.set_extension(STRATEGY_CONFIG_FILE_EXTENSION);
			if path.try_exists()? {
				continue;
			}
			let file = File::create(path)?;
			let mut serializer = strategy_serializer(file)?;
			let mut erased = <dyn erased_serde::Serializer>::erase(&mut serializer);
			strat.serialize(&mut erased)?;
		}
		
		Ok(())
	}
	
	pub fn extend(self, router: Router) -> Router {
		let mut strat_list = StrategyList::new();
		for strat in self.strategies {
			strat_list.add_from_container(strat);
		}
		
		tracing::info!("Setting up strategies: {:?}", strat_list);
		
		router
			.layer(Extension(strat_list))
			.layer(Extension(BatchTracker::default()))
	}
	
	pub fn extend_fn(self) -> impl FnOnce(Router) -> Router {
		move |router| self.extend(router)
	}
}