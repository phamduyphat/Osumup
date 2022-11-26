use crate::osu::client::ClientCredentials;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
	cell::RefCell,
	fs::File,
	io::{Read, Write},
	rc::Rc,
};

#[derive(Default, Serialize, Deserialize)]
pub struct Osu {
	pub client_id: u8,
	pub client_secret: String,
	pub credentials: ClientCredentials,
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
	pub osu_folder: String,
	pub verbose: bool,
	pub osu: Rc<RefCell<Osu>>,
}

impl Configuration {
	fn new() -> Configuration {
		Configuration {
			verbose: true,
			osu_folder: String::new(),
			osu: Rc::new(RefCell::new(Osu::default())),
		}
	}

	fn get_config_file() -> String {
		std::env::current_exe()
			.unwrap()
			.parent()
			.unwrap()
			.join("config.json")
			.to_str()
			.unwrap()
			.to_string()
	}

	pub fn load() -> Result<Configuration, std::io::Error> {
		let mut file = match File::open(Configuration::get_config_file()) {
			Ok(file) => file,
			Err(_) => {
				let config = Configuration::new();
				config.save()?;
				return Ok(config);
			}
		};
		let mut contents = String::new();
		file.read_to_string(&mut contents)?;

		Ok(serde_json::from_str(&contents)?)
	}

	pub fn save(&self) -> Result<(), std::io::Error> {
		let config = serde_json::to_string_pretty(&json!(&self))?;

		let mut file = File::create(Configuration::get_config_file())?;
		file.write_all(config.as_bytes())?;

		Ok(())
	}
}
