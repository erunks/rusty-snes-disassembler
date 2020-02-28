use std::process;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

use crate::opcode::Opcode;
use crate::opcode::handler;

pub struct Disassembler {
  opcodes: HashMap<String, Opcode>,
  rom: String,
}

impl Disassembler {
  pub fn new() -> Self {
    let opcodes = match handler::load_op_codes("./src/6502ops.csv") {
      Ok(opcodes) => opcodes,
      Err(e) => {
        println!("Opcodes failed to load: {}", e);
        process::exit(1)
      }
    };

    Disassembler {
      opcodes: opcodes,
      rom: "".to_string(),
    }
  }

  pub fn load_rom(&mut self, file_path: &str) -> Result<BufReader<File>, Box<dyn Error>> {
    self.rom = file_path.to_string();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    Ok(reader)
  }

  // fn disassemble_rom(&self) -> Result<(), Box<dyn Error>> {
  //   let rom : BufReader = self.rom;
  // }
}
