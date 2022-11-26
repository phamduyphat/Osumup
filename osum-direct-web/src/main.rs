#![feature(type_ascription)]
// #![windows_subsystem = "windows"]

use configuration::Configuration;
use osu::{client::OsuClient, launcher::Launcher};
use registry::Registry;
use std::{path::Path, rc::Rc};

mod cli;
mod configuration;
mod osu;
mod registry;

fn main() {
	let mut config = match Configuration::load() {
		Ok(config) => {
			if config.verbose {
				cli::log("Configuration loaded.");
			}
			config
		}
		Err(error) => {
			cli::ask(format!("Could not load configuration. This may be due to a malformed syntax. You can probably fix this by deleting the config.json file manually and run this program again.\nCause: {}", error));
			return;
		}
	};

	let registry = Registry::new();

	if let Err(error) = registry.create_entry() {
		cli::ask(format!("Could not create custom protocol handler. Please run with administative permissions.\nCause: {}", error));
		return;
	}

	if config.verbose {
		cli::log("Registry configuration is set up correctly.");
	}

	let mut client = match OsuClient::new(Rc::clone(&config.osu)) {
		Ok(client) => client,
		Err(_) => {
			cli::ask("Please enter your osu! client_id and client_secret in the newly generated config.json file.");
			return;
		}
	};

	let mut pending_change = false;

	'authorization: loop {
		if config.osu.borrow().credentials.access_token.is_empty() {
			loop {
				let login = client.login(
					cli::ask("Enter your username:"),
					cli::ask("Enter your password:"),
				);

				cli::clear();

				if let Err(error) = login {
					cli::ask(format!(
						"Authorized failed. Press enter to try again.\nCause: {}",
						error
					));
					continue;
				}

				if config.verbose {
					cli::log("Authorization successfull.");
				}

				pending_change = true;

				break 'authorization;
			}
		} else if client.is_expired() {
			if let Err(error) = client.refresh_token() {
				cli::ask(format!(
					"Refreshing token failed. Press enter to login.\nCause: {}",
					error
				));
				continue;
			}

			pending_change = true;
		}

		break;
	}

	if config.osu_folder.is_empty() {
		match registry.get_install_path() {
			Ok(path) => config.osu_folder = path,
			Err(error) => {
				cli::ask(format!(
					"Could not find osu! installation path automatically.\nCause: {}",
					error
				));
				config.osu_folder = cli::ask("Enter the path to the folder manually: ");
			}
		}
		pending_change = true;
	}

	if pending_change {
		if let Err(error) = config.save() {
			cli::ask(format!(
				"Could not save configuration to file.\nCause: {}",
				error
			));
			return;
		}

		if config.verbose {
			cli::log("Configuration is set up.");
		}
	}

	if config.verbose {
		cli::log("Configuration initialized.");
	}

	let args: Vec<String> = std::env::args().collect();

	if config.verbose && args.len() != 2 {
		cli::log("Ready. From now on you don't have to open this application anymore and you can use the osum!direct button.");
		return;
	}

	let mut launcher = Launcher::new(&config.osu_folder);

	if let Err(error) = launcher.run("") {
		cli::ask(format!("Could not start osu!.\nCause: {}", error));
		return;
	}

	let beatmap_id = args[1].split('?').nth(1).unwrap().parse::<u32>().unwrap();

	if config.verbose {
		cli::log(format!(
			"Launched osu!. Proceeding to download {}.",
			beatmap_id
		));
	}

	match client.download_beatmap(beatmap_id, Path::new(&config.osu_folder).join("Songs/")) {
		Ok(file) => {
			if launcher.run(file.to_str().unwrap()).is_err() {
				cli::ask("Could not import beatmap to osu!.");
			}

			if config.verbose {
				cli::log(format!(
					"Finished downloading and importing {}.",
					beatmap_id
				));
			}
		}
		Err(error) => {
			cli::ask(&format!(
				"Failed to download {}.\nCause: {}",
				beatmap_id, error
			));
		}
	}
}
