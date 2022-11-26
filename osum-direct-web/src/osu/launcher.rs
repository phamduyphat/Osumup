use std::{path::Path, process::Command};

pub struct Launcher {
	pub command: Command,
}

impl Launcher {
	pub fn new(osu_folder: &str) -> Self {
		Launcher {
			command: Command::new(Path::new(osu_folder).join("osu!.exe")),
		}
	}

	pub fn run(&mut self, arg: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
		let mut command = &mut self.command;
		if !arg.is_empty() {
			command = self.command.args([arg]);
		}
		command.spawn()?;
		Ok(())
	}
}
