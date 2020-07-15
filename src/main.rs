/*
 * Affliction, the better version written with <3 in Rust
 * Copyright (C) 2020 0v3rl0w & contributors
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::{process::Command, str::{from_utf8, Utf8Error}};

fn get_devices() -> Result<Vec<String>, Utf8Error> {
	let output_utf8 = Command::new("wmic")
				.args(&["logicaldisk", "get", "name"])
				.output()
				.expect("Failed to run the command !");
	
	let output: &str = from_utf8(&output_utf8.stdout)?;
	let mut result: Vec<&str> = output.split_whitespace().collect::<Vec<&str>>();
	result.remove(0);

	let result: Vec<String> = result.into_iter().map(|x| String::from(x)).collect();
	Ok(result)
}

fn main() {
	if let Ok(result) = get_devices() {
		println!("{:?}", result);
	}
}
