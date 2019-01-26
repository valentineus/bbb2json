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

#[derive(Serialize, Clone)]
struct Record {
	external_id: String,
	meeting_id: String,
	meeting_name: String,
}

fn main() {
	let file = File::open("events.xml").unwrap();
	let file = BufReader::new(file);

	let parser = EventReader::new(file);

	let mut data = Record {
		external_id: "".to_string(),
		meeting_id: "".to_string(),
		meeting_name: "".to_string(),
	};

	for element in parser {
		match element {
			Ok(XmlEvent::StartElement {
				name: _,
				attributes,
				..
			}) => {
				for attribute in attributes {
					let name: String = attribute.name.local_name.to_string();
					let value: String = attribute.value.to_string();

					if name == "externalId" {
						data.external_id = value;
					} else if name == "meetingId" {
						data.meeting_id = value;
					} else if name == "meetingName" {
						data.meeting_name = value;
					}
				}
			}
			Err(error) => {
				dbg!(error);
				break;
			}
			_ => {}
		}
	}

	let j = serde_json::to_string(&data);
	dbg!(j);
}
