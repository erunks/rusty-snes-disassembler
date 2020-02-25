use std::error::Error;
use std::fs::File;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OpCode {
  opcode: i16,
  mnemonic: String,
  addressing_mode: String,
  bytes: i32,
  cycles: String,
  flags: String,
}

pub fn load_op_codes(file_path: &str) -> Result<HashMap<String, OpCode>, Box<Error>> {
  let file = File::open(file_path)?;
  let mut reader = csv::Reader::from_reader(file);
  let mut map: HashMap<String, OpCode> = HashMap::new();

  for result in reader.records() {
    let record = result?;
    let opcode: OpCode = record.deserialize(None)?;
    map.insert(record[0].to_string(), opcode);
  }

  Ok(map)
}