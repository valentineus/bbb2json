extern crate xml;

use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

pub struct ParserResult {
	pub external_id: String,
	pub meeting_id: String,
	pub meeting_name: String,
}

pub fn parse(content: BufReader<File>) -> ParserResult {
	let parser = EventReader::new(content);

	let mut data = ParserResult {
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

	return data;
}
