#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate xml;

use std::fs::File;
use std::io::BufReader;

use serde::Serialize;
use serde_json::Result;
use xml::reader::{EventReader, XmlEvent};

#[derive(Serialize)]
struct Record {
	externalId: String,
	meetingId: String,
	meetingName: String,
}

fn main() {
	let file = File::open("events.xml").unwrap();
	let file = BufReader::new(file);

	let parser = EventReader::new(file);

	for e in parser {
		match e {
			Ok(XmlEvent::StartElement {
				name: _,
				attributes,
				..
			}) => {
				for a in attributes {
					let mut data: Record;

					match a.name.local_name.as_ref() {
						"externalId" => data.externalId = a.value.to_owned(),
						"meetingId" => data.meetingId = a.value.to_owned(),
						"meetingName" => data.meetingName = a.value.to_owned(),
						_ => {}
					};

					let j = serde_json::to_string(&data);
					dbg!(j);
				}
			}
			Err(e) => {
				dbg!(e);
				break;
			}
			_ => {}
		}
	}
}
