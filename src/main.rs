use std::fs::File;
use std::io::BufReader;
use std::path::Path;

extern crate clap;
use clap::{load_yaml, App};

mod parser;
use parser::*;

fn main() {
	let yaml = load_yaml!("cli/ru.yml");
	let matches = App::from_yaml(yaml).get_matches();

	let path = matches.value_of("FILE").unwrap();

	match Path::new(path).exists() {
		true => {
			let file: File = File::open(path).unwrap();
			let data: ParserResult = parse(BufReader::new(file));
		}
		_ => panic!("Error File!"),
	};
}
