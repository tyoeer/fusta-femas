use std::path::PathBuf;

use serde::Deserialize;


const ENVIRONMENT_VARIABLE_PREFIX: &str = "FUSTA_FEMAS_";
const DEFAULT_DATABASE_FILE: &str = "content.db";


#[derive(Clone, Deserialize)]
pub struct Config {
	///The path to the folder in which all data should be stored
	pub data_path: Option<PathBuf>,
	///The path at which the database file is located.
	pub database_path: Option<PathBuf>,
}

impl Config {
	pub fn load() -> Self {
		envy::prefixed(ENVIRONMENT_VARIABLE_PREFIX).from_env().expect("Environment variables should have loaded")
	}
}

#[derive(Debug)]
pub struct Settings {
	pub data_path: PathBuf,
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
			data_path: maybe_data_path,
			database_path: maybe_database_path, 
		} = config;
		
		let data_path = maybe_data_path.unwrap_or_else(|| {
			let mut path = PathBuf::new();
			path.push(".local-ff-data");
			path.push("dev");
			path
		});
		
		let database_path = maybe_database_path.unwrap_or_else(|| {
			let mut path = data_path.clone();
			path.push(DEFAULT_DATABASE_FILE);
			path
		});
		
		let db = database_path.into_os_string().into_string().expect("database path should be valid utf8");
		//Can't be parsed otherwise. TODO: find less hacky way of doing this
		let db = db.replace('\\', "/");
		let database_url = format!("sqlite://{db}?mode=rwc");
		
		Self {
			data_path,
			database_url,
		}
	}
}