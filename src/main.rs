#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate clap;
extern crate xml;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use clap::{load_yaml, App};

mod parser;
use parser::*;

fn main() {
	let yaml = load_yaml!("cli/en.yml");
	let matches = App::from_yaml(yaml).get_matches();

	let path = matches.value_of("FILE").unwrap();

	match Path::new(path).exists() {
		true => {
			let file: File = File::open(path).unwrap();
			let data: ParserResult = parser(BufReader::new(file));
			println!("{:#?}", serde_json::to_string_pretty(&data));
		}
		_ => panic!("File does not exist or is not available."),
	};
}
