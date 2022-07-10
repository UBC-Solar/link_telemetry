extern crate yaml_rust;

use can::StandardFrame;
use std::{error::Error, path::PathBuf, time::Instant};

mod can;
mod measurement;
mod parsing;

fn main() {
    // read CAN string
    // let test_string = String::from("DDDDDDDD05014120000041a000008\n");

    // 0x626
    let test_string = String::from("DDDDDDDD06279A20000041a000008\n");

    let frame = StandardFrame::new(test_string).unwrap();

    println!("{:#?}", frame);

    let yaml_path = PathBuf::from("can-rust.yaml").canonicalize().unwrap();

    // create parser
    let parser = parsing::Parser::new(yaml_path).unwrap();

    // extract measurements
    let extracted_measurements = parser.parse(frame);

    println!("Extracted measurements: {:#?}", extracted_measurements);
}
