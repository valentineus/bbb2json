use std::fs::File;
use std::io::BufReader;

mod parser;
use parser::*;

fn main() {
	let file: File = File::open("events.xml").unwrap();
	let data: ParserResult = qwerty(BufReader::new(file));
	dbg!(data.meeting_name);
}
