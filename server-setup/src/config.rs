use std::path::PathBuf;

use serde::Deserialize;


const ENVIRONMENT_VARIABLE_PREFIX: &str = "FUSTA_FEMAS_";


#[derive(Clone, Deserialize)]
pub struct Config {
	// pub save_path: Option<PathBuf>,
	pub database_path: Option<PathBuf>,
}

impl Config {
	pub fn load() -> Self {
		envy::prefixed(ENVIRONMENT_VARIABLE_PREFIX).from_env().expect("Environment variables should have loaded")
	}
}

#[derive(Debug)]
pub struct Settings {
	// pub save_path: PathBuf,
	pub database_url: String,
}

impl Settings {
	pub fn load() -> Self {
		Config::load().into()
	}
}

impl From<Config> for Settings {
	fn from(config: Config) -> Self {
		let Config {
			// save_path: maybe_save_path,
			database_path: maybe_database_path, 
		} = config;
		
		// let save_path = maybe_save_path //TODO
		
		// let database_path = maybe_database_path.unwrap_or_else(|| {
		// 	let mut path = save_path.clone();
		// 	path.push("database");
		// 	path
		// });
		let database_path = maybe_database_path.unwrap();
		
		let db = database_path.into_os_string().into_string().expect("database path should be valid utf8");
		let database_url = format!("sqlite://{db}?mode=rwc");
		
		Self {
			// save_path,
			database_url,
		}
	}
}