use std::path::PathBuf;

use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

use std::fs;

use crate::can::StandardFrame;
use crate::measurement::Measurement;
use crate::measurement::MeasurementValue;

#[derive(Debug)]
pub struct Parser {
    schema_path: PathBuf,
    schema: Yaml,
}

#[derive(Debug)]
pub enum ParseType {
    IEEE32Float,
    Bool,
    Unsigned16,
    Unsigned8,
    Incremental,
    Signed16,
    Signed8,
}

impl ParseType {
    pub fn into_parse_type(string: &str) -> Result<ParseType, ()> {
        match string {
            "ieee32_float" => Ok(ParseType::IEEE32Float),
            "bool" => Ok(ParseType::Bool),
            "unsigned_16" => Ok(ParseType::Unsigned16),
            "unsigned_8" => Ok(ParseType::Unsigned8),
            "incremental" => Ok(ParseType::Incremental),
            "signed_16" => Ok(ParseType::Signed16),
            "signed_8" => Ok(ParseType::Signed8),
            _ => Err(()),
        }
    }
}

impl Parser {
    pub fn new(schema_path: PathBuf) -> Result<Self, std::io::Error> {
        // read YAML from path into string
        let read_result = fs::read_to_string(&schema_path);

        // TODO: add verification of schema here

        match read_result {
            Ok(schema_string) => {
                // parse YAML from string
                let docs = YamlLoader::load_from_str(&schema_string).unwrap();

                // extract schema as first document
                let schema = &docs[0];

                Ok(Self {
                    schema_path,
                    schema: schema.clone(),
                })
            }
            Err(e) => Err(e),
        }
    }

    pub fn parse(&self, frame: StandardFrame) -> Vec<Measurement> {
        // vector containing all parsed measurements from the StandardFrame
        let mut ms_vec: Vec<Measurement> = Vec::new();

        let schema_for_id = self.schema[frame.id() as usize].clone();

        // unwrapping these shouldn't be an issue if the YAML file is constructed properly
        let source = schema_for_id["source"].as_str().unwrap();
        let msg_name = schema_for_id["name"].as_str().unwrap();
        let measurement_schema = schema_for_id["measurements"].as_vec().unwrap();

        for ms in measurement_schema {
            let mut hash = ms.as_hash().unwrap().clone();
            for entry in hash.entries() {
                // name of the measurement
                let ms_name = entry.key().as_str().unwrap();

                // information about how to parse the measurement
                let extraction_data = entry.get();

                let bits = extraction_data["bits"].as_vec().unwrap();
                let lower_bit = bits.get(0).unwrap().as_i64().unwrap() as u8;

                let upper_bit_result = bits.get(1);

                let mut upper_bit = lower_bit;

                // upper bit value is optional, if not found
                // assume equality to lower bit value
                if let Some(ub) = upper_bit_result {
                    upper_bit = ub.as_i64().unwrap() as u8;
                }

                let ms_type = extraction_data["type"].as_str().unwrap();

                let parse_type = ParseType::into_parse_type(ms_type).unwrap();

                // extract measurement value
                let value = Self::extract(frame.payload(), parse_type, lower_bit, upper_bit);

                // package into Measurement struct
                let new_measurement =
                    Measurement::new(frame.id(), msg_name.to_string(), source.to_string(), ms_name.to_string(), value);

                // add to return vector
                ms_vec.push(new_measurement);
            }
        }

        ms_vec
    }

    pub fn extract(payload: u64, p_type: ParseType, lower: u8, upper: u8) -> MeasurementValue {
        match p_type {
            ParseType::IEEE32Float => {
                assert!(
                    upper - lower == 31,
                    "difference between bit bounds must be 31 for this type"
                );
                let value_bits = payload >> (64 - (upper + 1)) & 0xFFFFFFFF;
                let value_bytes = (value_bits as u32).to_le_bytes();
                MeasurementValue::IEEE32Float(f32::from_le_bytes(value_bytes))
            }
            ParseType::Bool => {
                assert_eq!(upper, lower, "bit bounds must be equal for this type");

                let flag = payload >> (64 - (upper + 1)) & 0x1;

                match flag {
                    0 => MeasurementValue::Bool(false),
                    1 => MeasurementValue::Bool(true),
                    _ => MeasurementValue::Invalid
                }
            }
            ParseType::Unsigned8 => {
                assert!(
                    upper - lower == 7,
                    "difference between bit bounds must be 7 for this type"
                );
                let value = payload >> (64 - (upper + 1)) & 0xFF;
                MeasurementValue::Unsigned8(value as u8)
            },
            ParseType::Unsigned16 => {
                assert!(
                    upper - lower == 15,
                    "difference between bit bounds must be 15 for this type"
                );
                let value = payload >> (64 - (upper + 1)) & 0xFFFF;
                MeasurementValue::Unsigned16(value as u16)
            },
            ParseType::Signed8 => {
                assert!(
                    upper - lower == 7,
                    "difference between bit bounds must be 7 for this type"
                );
                let value = payload >> (64 - (upper + 1)) & 0xFF;
                MeasurementValue::Signed8(value as i8)
            },
            ParseType::Signed16 => {
                assert!(
                    upper - lower == 15,
                    "difference between bit bounds must be 15 for this type"
                );
                let value = payload >> (64 - (upper + 1)) & 0xFF;
                MeasurementValue::Signed16(value as i16)
            },
            ParseType::Incremental => {
                assert!(
                    upper - lower == 7,
                    "difference between bit bounds must be 7 for this type"
                );
                let value = payload >> (64 - (upper + 1)) & 0xFFFF_FFFF;
                MeasurementValue::Incremental(value as f32 * 0.1)
            }
        }
    }
}
