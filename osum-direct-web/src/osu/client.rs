use crate::configuration::Osu;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::{
	cell::RefCell,
	fs::File,
	io::{Error, ErrorKind, Write},
	path::PathBuf,
	rc::Rc,
	time::{Duration, SystemTime},
};

pub struct OsuClient {
	pub osu_config: Rc<RefCell<Osu>>,
	pub http_client: Client,
}

impl OsuClient {
	pub fn new(osu_config: Rc<RefCell<Osu>>) -> Result<OsuClient, Error> {
		let config = osu_config.borrow();

		if config.client_secret.is_empty() || config.client_id == 0 {
			return Err(ErrorKind::InvalidData.into());
		}

		drop(config);

		Ok(OsuClient {
			osu_config,
			http_client: Client::new(),
		})
	}

	pub fn download_beatmap(
		&self,
		beatmap_id: u32,
		path: PathBuf,
	) -> Result<PathBuf, Box<dyn std::error::Error>> {
		let config = self.osu_config.borrow();
		let response = self
			.http_client
			.get(format!(
				"https://osu.ppy.sh/api/v2/beatmapsets/{}/download",
				beatmap_id
			))
			.header(
				"Authorization",
				"Bearer ".to_string() + &config.credentials.access_token,
			)
			.send()?;

		if !response.status().is_success() {
			return Err(Box::new(Error::new(
				ErrorKind::Other,
				"Response was unsucessfull.",
			)));
		}

		if let Some((_, header_value)) = response
			.headers()
			.iter()
			.find(|&(header_name, _)| header_name == "Content-Disposition")
		{
			let file_name = sanitize_filename::sanitize(
				header_value.to_str()?.split('\"').collect::<Vec<&str>>()[1],
			);
			let path = path.join(file_name);
			if !path.exists() {
				let mut file = File::create(path.clone())?;
				file.write_all(&response.bytes()?)?;
			}
			return Ok(path);
		}
		Ok(PathBuf::new())
	}

	pub fn is_expired(&self) -> bool {
		let now = SystemTime::now()
			.duration_since(std::time::UNIX_EPOCH)
			.unwrap()
			.as_secs();

		return self.osu_config.borrow().credentials.expires_in < now;
	}

	fn update_credentials<T: Serialize>(&self, json: T) -> Result<(), Box<dyn std::error::Error>> {
		let response = self
			.http_client
			.post("https://osu.ppy.sh/oauth/token")
			.json(&json)
			.send()?;

		let osu_config = &mut self.osu_config.borrow_mut();

		if !response.status().is_success() {
			osu_config.credentials = ClientCredentials::default();

			return Err(Box::new(Error::new(
				ErrorKind::Other,
				"Authorization failed.",
			)));
		}

		let new_credentials: ClientCredentials = response.json()?;

		osu_config.credentials.access_token = new_credentials.access_token;
		osu_config.credentials.refresh_token = new_credentials.refresh_token;

		let expires_at = SystemTime::now() + Duration::new(new_credentials.expires_in, 0);
		osu_config.credentials.expires_in = expires_at
			.duration_since(std::time::UNIX_EPOCH)
			.unwrap()
			.as_secs();

		Ok(())
	}

	pub fn refresh_token(&self) -> Result<(), Box<dyn std::error::Error>> {
		let osu_config = self.osu_config.borrow();

		let json = RefreshRequest {
			client_id: osu_config.client_id,
			client_secret: osu_config.client_secret.clone(),
			refresh_token: osu_config.credentials.refresh_token.clone(),
			grant_type: "refresh_token".to_string(),
			scope: "*".to_string(),
		};

		drop(osu_config);

		self.update_credentials(json)?;

		Ok(())
	}

	pub fn login(
		&mut self,
		username: String,
		password: String,
	) -> Result<(), Box<dyn std::error::Error>> {
		let osu_config = self.osu_config.borrow();

		let json = LoginRequest {
			client_id: osu_config.client_id,
			client_secret: osu_config.client_secret.clone(),
			username,
			password,
			grant_type: "password".to_string(),
			scope: "*".to_string(),
		};

		drop(osu_config);

		self.update_credentials(json)?;

		Ok(())
	}
}

#[derive(Serialize, Deserialize)]
struct LoginRequest {
	client_id: u8,
	client_secret: String,
	username: String,
	password: String,
	grant_type: String,
	scope: String,
}

#[derive(Serialize, Deserialize)]
struct RefreshRequest {
	client_id: u8,
	client_secret: String,
	refresh_token: String,
	grant_type: String,
	scope: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct ClientCredentials {
	pub access_token: String,
	pub refresh_token: String,
	pub expires_in: u64,
}
