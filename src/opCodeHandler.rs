use std::error::Error;
use std::fs::File;
use std::collections::HashMap;

struct OpCode<'a> {
  opcode: i16,
  mnemonic: &'a str,
  addressing_mode: &'a str,
  bytes: i32,
  cycles: &'a str,
  flags: &'a str,
}

pub fn load_op_codes(file_path: &str) -> Result<(), Box<Error>> {
  let file = File::open(file_path)?;
  let mut reader = csv::Reader::from_reader(file);
  for result in reader.records() {
    let record = result?;
    println!("{:?}", record);
  }
  Ok(())
}