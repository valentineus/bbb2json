use std::fs::File;
use std::io::BufReader;

use serde_json::Value;
use xml::reader::{EventReader, XmlEvent};

#[derive(Serialize)]
pub struct ParserResult {
	pub external_id: String,
	pub meeting_id: String,
	pub meeting_name: String,
	pub context: Value,
}

pub fn parser(content: BufReader<File>) -> ParserResult {
	let mut data = ParserResult {
		context: serde_json::from_str("{}").unwrap(),
		external_id: "".to_string(),
		meeting_id: "".to_string(),
		meeting_name: "".to_string(),
	};

	for element in EventReader::new(content) {
		match element {
			Ok(XmlEvent::StartElement {
				name, attributes, ..
			}) => {
				let el_name = name.local_name.to_string();

				for attribute in attributes {
					let attr_name: String = attribute.name.local_name.to_string();
					let attr_value: String = attribute.value.to_string();

					if el_name == "meeting" && attr_name == "id" {
						data.meeting_id = attr_value.clone();
					}

					if el_name == "meeting" && attr_name == "externalId" {
						data.external_id = attr_value.clone();
					}

					if el_name == "meeting" && attr_name == "name" {
						data.meeting_name = attr_value.clone();
					}

					if el_name == "metadata" && attr_name == "bn-recording-status" {
						data.context = serde_json::from_str(&attr_value).unwrap();
					}
				}
			}
			Err(error) => panic!(error),
			_ => {}
		}
	}

	return data;
}
