use std::{fmt::Display, io::stdin, time::SystemTime};

use chrono::{DateTime, Utc};

pub fn ask<T: Display>(message: T) -> String {
	println!("{}", message);
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();
	input[0..input.len() - 2].to_string()
}

pub fn log<T: Display>(message: T) {
	println!(
		"[{}] {}",
		(SystemTime::now().into(): DateTime<Utc>).format("%d/%m/%Y %T"),
		message
	);
}
pub fn clear() {
	clearscreen::clear().unwrap();
}
