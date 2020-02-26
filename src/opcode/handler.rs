use std::error::Error;
use std::fs::File;
use std::collections::HashMap;

use crate::opcode::Opcode;

pub fn load_op_codes(file_path: &str) -> Result<HashMap<String, Opcode>, Box<dyn Error>> {
  let file = File::open(file_path)?;
  let mut reader = csv::Reader::from_reader(file);
  let mut map: HashMap<String, Opcode> = HashMap::new();

  for result in reader.records() {
    let record = result?;
    let opcode: Opcode = record.deserialize(None)?;
    map.insert(record[0].to_string(), opcode);
  }

  Ok(map)
}