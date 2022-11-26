use std::{
	env::current_exe,
	io::{Error, ErrorKind},
};
use winreg::{enums::*, RegKey};

pub struct Registry {
	hcr: RegKey,
	default_path: String,
	exe_path: String,
}

impl Registry {
	pub fn new() -> Registry {
		let hcr = RegKey::predef(HKEY_CLASSES_ROOT);
		Registry {
			hcr,
			default_path: r#"osum-direct-web\shell\open\command"#.to_string(),
			exe_path: current_exe().unwrap().to_str().unwrap().to_string(),
		}
	}

	pub fn get_install_path(&self) -> Result<String, Error> {
		let path = self
			.hcr
			.open_subkey_with_flags(r#"osu\shell\open\command"#, KEY_READ)?
			.get_value::<String, &str>("")?
			.split("osu!.exe")
			.collect::<Vec<&str>>()[0]
			.to_string();
		Ok(path)
	}

	pub fn create_entry(&self) -> Result<(), Error> {
		if !self.handler_exists().or_else(|error| {
			if error.kind() != ErrorKind::NotFound {
				Err(error)
			} else {
				Ok(false)
			}
		})? {
			self.set_handler()?;
		}
		Ok(())
	}

	fn handler_exists(&self) -> Result<bool, Error> {
		let hcr_key = self
			.hcr
			.open_subkey_with_flags(self.default_path.to_owned(), KEY_READ)?;

		let value = hcr_key.get_value::<String, &str>("")?;
		if value.is_empty() || self.exe_path != value[0..value.len() - 3] {
			return Ok(false);
		}
		Ok(true)
	}

	fn set_handler(&self) -> Result<(), Error> {
		let (key, _) = self.hcr.create_subkey(self.default_path.to_owned())?;
		key.set_value("", &(self.exe_path.to_owned() + " %1"))?;

		let root_key = self
			.hcr
			.open_subkey_with_flags("osum-direct-web", KEY_WRITE)?;
		root_key.set_value("", &"URL:osum-direct-web")?;
		root_key.set_value("URL Protocol", &"")?;

		Ok(())
	}
}
