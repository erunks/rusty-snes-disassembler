use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

use crate::opcode::Opcode;
use crate::opcode::handler::load_op_codes;

pub struct Disassembler {
  opcodes: HashMap<String, Opcode>,
  rom: String,
}

impl Disassembler {
  pub fn new() -> Disassembler {
    let opcodes = match load_op_codes("./opcodes/6502ops.csv") {
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

  pub fn load_rom(&mut self, file_path: &str) {
    self.rom = file_path.to_string();
  }

  pub fn disassemble_rom(&self) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(&self.rom)?;
    
    /* read the entire file into the buffer,
     * so that we can step through it later
     */
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;

    let file_size: i64 = i64::try_from(buffer.len()).unwrap();
    let mut pointer_counter: i64 = 0;
    while pointer_counter < file_size {
      pointer_counter += self.lookup_opcode_instruction(&buffer, pointer_counter);
    }
    Ok(())
  }

  fn lookup_opcode_instruction(&self, file_buffer: &Vec<u8>, pointer_counter: i64) -> i64 {
    #[allow(unused_mut)]
    let mut opstring: String;
    let opcodes = |position: i64| -> u8 { 
      let new_position: usize = usize::try_from(pointer_counter + position).unwrap();
      file_buffer[new_position]
    };
    let mut count: i64 = 1;

    match opcodes(0) {
      0x00 => { opstring = String::from("BRK"); },
      0x01 => { opstring = format!("ORA (${:02x},X)", opcodes(1)); count = 2; },
      0x05 => { opstring = format!("ORA ${:02x}", opcodes(1)); count = 2; },
      0x06 => { opstring = format!("ASL ${:02x}", opcodes(1)); count = 2; },
      0x08 => { opstring = String::from("PHP"); },
      0x09 => { opstring = format!("ORA #${:02x}", opcodes(1)); count = 2; },
      0x0a => { opstring = String::from("ASL A"); },
      0x0d => { opstring = format!("ORA ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0x0e => { opstring = format!("ASL ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0x10 => { opstring = format!("BPL ${:02x}", opcodes(1)); count = 2; },
      0x11 => { opstring = format!("ORA (${:02x}),Y", opcodes(1)); count = 2; },
      0x15 => { opstring = format!("ORA ${:02x},X", opcodes(1)); count = 2; },
      0x16 => { opstring = format!("ASL ${:02x},X", opcodes(1)); count = 2; },
      0x18 => { opstring = String::from("CLC"); },
      0x19 => { opstring = format!("ORA ${:02x}{:02x},Y", opcodes(2), opcodes(1)); count = 3; },
      0x1d => { opstring = format!("ORA ${:02x}{:02x},X", opcodes(2), opcodes(1)); count = 3; },
      0x1e => { opstring = format!("ASL ${:02x}{:02x},X", opcodes(2), opcodes(1)); count = 3; },
      0x20 => { opstring = format!("JSR ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0x21 => { opstring = format!("AND (${:02x},X)", opcodes(1)); count = 2; },
      0x24 => { opstring = format!("BIT ${:02x}", opcodes(1)); count = 2; },
      0x25 => { opstring = format!("AND ${:02x}", opcodes(1)); count = 2; },
      0x26 => { opstring = format!("ROL ${:02x}", opcodes(1)); count = 2; },
      0x28 => { opstring = String::from("PLP"); },
      0x29 => { opstring = format!("AND #${:02x}", opcodes(1)); count = 2; },
      0x2a => { opstring = String::from("ROL A"); },
      0x2c => { opstring = format!("BIT ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0x2d => { opstring = format!("AND ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0x2e => { opstring = format!("ROL ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0x30 => { opstring = format!("BMI ${:02x}", opcodes(1)); count = 2; },
      0x31 => { opstring = format!("AND (${:02x}),Y", opcodes(1)); count = 2; },
      0x35 => { opstring = format!("AND ${:02x},X", opcodes(1)); count = 2; },
      0x36 => { opstring = format!("ROL ${:02x},X", opcodes(1)); count = 2; },
      0x38 => { opstring = String::from("SEC"); },
      0x39 => { opstring = format!("AND ${:02x}{:02x},Y", opcodes(2), opcodes(1)); count = 3; },
      0x3d => { opstring = format!("AND ${:02x}{:02x},X", opcodes(2), opcodes(1)); count = 3; },
      0x3e => { opstring = format!("ROL ${:02x}{:02x},X", opcodes(2), opcodes(1)); count = 3; },
      0x40 => { opstring = String::from("RTI"); },
      0x41 => { opstring = format!("EOR (${:02x},X)", opcodes(1)); count = 2; },
      0x45 => { opstring = format!("EOR ${:02x}", opcodes(1)); count = 2; },
      0x46 => { opstring = format!("LSR ${:02x}", opcodes(1)); count = 2; },
      0x48 => { opstring = String::from("PHA"); },
      0x49 => { opstring = format!("EOR #${:02x}", opcodes(1)); count = 2; },
      0x4a => { opstring = String::from("LSR A"); },
      0x4c => { opstring = format!("JMP ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0x4d => { opstring = format!("EOR ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0x4e => { opstring = format!("LSR ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0x50 => { opstring = format!("BVC ${:02x}", opcodes(1)); count = 2; },
      0x51 => { opstring = format!("EOR (${:02x}),Y", opcodes(1)); count = 2; },
      0x55 => { opstring = format!("EOR ${:02x},X", opcodes(1)); count = 2; },
      0x56 => { opstring = format!("LSR ${:02x},X", opcodes(1)); count = 2; },
      0x58 => { opstring = String::from("CLI"); },
      0x59 => { opstring = format!("EOR ${:02x}{:02x},Y", opcodes(2), opcodes(1)); count = 3; },
      0x5d => { opstring = format!("EOR ${:02x}{:02x},X", opcodes(2), opcodes(1)); count = 3; },
      0x5e => { opstring = format!("LSR ${:02x}{:02x},X", opcodes(2), opcodes(1)); count = 3; },
      0x60 => { opstring = String::from("RTS"); },
      0x61 => { opstring = format!("ADC (${:02x},X)", opcodes(1)); count = 2; },
      0x65 => { opstring = format!("ADC ${:02x}", opcodes(1)); count = 2; },
      0x66 => { opstring = format!("ROR ${:02x}", opcodes(1)); count = 2; },
      0x68 => { opstring = String::from("PLA"); },
      0x69 => { opstring = format!("ADC #${:02x}", opcodes(1)); count = 2; },
      0x6a => { opstring = String::from("ROR A"); },
      0x6c => { opstring = format!("JMP (${:02x}{:02x})", opcodes(2), opcodes(1)); count = 3; },
      0x6d => { opstring = format!("ADC ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0x6e => { opstring = format!("ROR ${:02x}{:02x},X", opcodes(2), opcodes(1)); count = 3; },
      0x70 => { opstring = format!("BVS ${:02x}", opcodes(1)); count = 2; },
      0x71 => { opstring = format!("ADC (${:02x}),Y", opcodes(1)); count = 2; },
      0x75 => { opstring = format!("ADC ${:02x},X", opcodes(1)); count = 2; },
      0x76 => { opstring = format!("ROR ${:02x},X", opcodes(1)); count = 2; },
      0x78 => { opstring = String::from("SEI"); },
      0x79 => { opstring = format!("ADC ${:02x}{:02x},Y", opcodes(2), opcodes(1)); count = 3; },
      0x7d => { opstring = format!("ADC ${:02x}{:02x},X", opcodes(2), opcodes(1)); count = 3; },
      0x7e => { opstring = format!("ROR ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0x81 => { opstring = format!("STA (${:02x},X)", opcodes(1)); count = 2; },
      0x84 => { opstring = format!("STY ${:02x}", opcodes(1)); count = 2; },
      0x85 => { opstring = format!("STA ${:02x}", opcodes(1)); count = 2; },
      0x86 => { opstring = format!("STX ${:02x}", opcodes(1)); count = 2; },
      0x88 => { opstring = String::from("DEY"); },
      0x8a => { opstring = String::from("TXA"); },
      0x8c => { opstring = format!("STY ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0x8d => { opstring = format!("STA ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0x8e => { opstring = format!("STX ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0x90 => { opstring = format!("BCC ${:02x}", opcodes(1)); count = 2; },
      0x91 => { opstring = format!("STA (${:02x}),Y", opcodes(1)); count = 2; },
      0x94 => { opstring = format!("STY ${:02x},X", opcodes(1)); count = 2; },
      0x95 => { opstring = format!("STA ${:02x},X", opcodes(1)); count = 2; },
      0x96 => { opstring = format!("STX ${:02x},Y", opcodes(1)); count = 2; },
      0x98 => { opstring = String::from("TYA"); },
      0x99 => { opstring = format!("STA ${:02x}{:02x},Y", opcodes(2), opcodes(1)); count = 3; },
      0x9a => { opstring = String::from("TXS"); },
      0x9d => { opstring = format!("STA ${:02x}{:02x},X", opcodes(2), opcodes(1)); count = 3; },
      0xa0 => { opstring = format!("LDY #${:02x}", opcodes(1)); count = 2; },
      0xa1 => { opstring = format!("LDA (${:02x},X)", opcodes(1)); count = 2; },
      0xa2 => { opstring = format!("LDX #${:02x}", opcodes(1)); count = 2; },
      0xa4 => { opstring = format!("LDY ${:02x}", opcodes(1)); count = 2; },
      0xa5 => { opstring = format!("LDA ${:02x}", opcodes(1)); count = 2; },
      0xa6 => { opstring = format!("LDX ${:02x}", opcodes(1)); count = 2; },
      0xa8 => { opstring = String::from("TAY"); },
      0xa9 => { opstring = format!("LDA #${:02x}", opcodes(1)); count = 2; },
      0xaa => { opstring = String::from("TAX"); },
      0xac => { opstring = format!("LDY ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0xad => { opstring = format!("LDA ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0xae => { opstring = format!("LDX ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0xb0 => { opstring = format!("BCS ${:02x}", opcodes(1)); count = 2; },
      0xb1 => { opstring = format!("LDA (${:02x}),Y", opcodes(1)); count = 2; },
      0xb4 => { opstring = format!("LDY ${:02x},X", opcodes(1)); count = 2; },
      0xb5 => { opstring = format!("LDA ${:02x},X", opcodes(1)); count = 2; },
      0xb6 => { opstring = format!("LDX ${:02x},Y", opcodes(1)); count = 2; },
      0xb8 => { opstring = String::from("CLV"); },
      0xb9 => { opstring = format!("LDA ${:02x}{:02x},Y", opcodes(2), opcodes(1)); count = 3; },
      0xba => { opstring = String::from("TSX"); },
      0xbc => { opstring = format!("LDY ${:02x}{:02x},X", opcodes(2), opcodes(1)); count = 3; },
      0xbd => { opstring = format!("LDA ${:02x}{:02x},X", opcodes(2), opcodes(1)); count = 3; },
      0xbe => { opstring = format!("LDX ${:02x}{:02x},Y", opcodes(2), opcodes(1)); count = 3; },
      0xc0 => { opstring = format!("CPY #${:02x}", opcodes(1)); count = 2; },
      0xc1 => { opstring = format!("CMP (${:02x},X)", opcodes(1)); count = 2; },
      0xc4 => { opstring = format!("CPY ${:02x}", opcodes(1)); count = 2; },
      0xc5 => { opstring = format!("CMP ${:02x}", opcodes(1)); count = 2; },
      0xc6 => { opstring = format!("DEC ${:02x}", opcodes(1)); count = 2; },
      0xc8 => { opstring = String::from("INY"); },
      0xc9 => { opstring = format!("CMP #${:02x}", opcodes(1)); count = 2; },
      0xca => { opstring = String::from("DEX"); },
      0xcc => { opstring = format!("CPY ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0xcd => { opstring = format!("CMP ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0xce => { opstring = format!("DEC ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0xd0 => { opstring = format!("BNE ${:02x}", opcodes(1)); count = 2; },
      0xd1 => { opstring = format!("CMP (${:02x}),Y", opcodes(1)); count = 2; },
      0xd5 => { opstring = format!("CMP ${:02x},X", opcodes(1)); count = 2; },
      0xd6 => { opstring = format!("DEC ${:02x},X", opcodes(1)); count = 2; },
      0xd8 => { opstring = String::from("CLD"); },
      0xd9 => { opstring = format!("CMP ${:02x}{:02x},Y", opcodes(2), opcodes(1)); count = 3; },
      0xdd => { opstring = format!("CMP ${:02x}{:02x},X", opcodes(2), opcodes(1)); count = 3; },
      0xde => { opstring = format!("DEC ${:02x}{:02x},X", opcodes(2), opcodes(1)); count = 3; },
      0xe0 => { opstring = format!("CPX #${:02x}", opcodes(1)); count = 2; },
      0xe1 => { opstring = format!("SBC (${:02x},X)", opcodes(1)); count = 2; },
      0xe4 => { opstring = format!("CPX ${:02x}", opcodes(1)); count = 2; },
      0xe5 => { opstring = format!("SBC ${:02x}", opcodes(1)); count = 2; },
      0xe6 => { opstring = format!("INC ${:02x}", opcodes(1)); count = 2; },
      0xe8 => { opstring = String::from("INX"); },
      0xe9 => { opstring = format!("SBC #${:02x}", opcodes(1)); count = 2; },
      0xea => { opstring = String::from("NOP"); },
      0xec => { opstring = format!("CPX ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0xed => { opstring = format!("SBC ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0xee => { opstring = format!("INC ${:02x}{:02x}", opcodes(2), opcodes(1)); count = 3; },
      0xf0 => { opstring = format!("BEQ ${:02x}", opcodes(1)); count = 2; },
      0xf1 => { opstring = format!("SBC (${:02x}),Y", opcodes(1)); count = 2; },
      0xf5 => { opstring = format!("SBC ${:02x},X", opcodes(1)); count = 2; },
      0xf6 => { opstring = format!("INC ${:02x},X", opcodes(1)); count = 2; },
      0xf8 => { opstring = String::from("SED"); },
      0xf9 => { opstring = format!("SBC ${:02x}{:02x},Y", opcodes(2), opcodes(1)); count = 3; },
      0xfd => { opstring = format!("SBC ${:02x}{:02x},X", opcodes(2), opcodes(1)); count = 3; },
      0xfe => { opstring = format!("INC ${:02x}{:02x},X", opcodes(2), opcodes(1)); count = 3; },
      _ => { opstring = format!(".db ${:02x}", opcodes(0)); }
    }
    print!("{:04X} {:02X} ", 0x5000 + pointer_counter, opcodes(0));

    if count > 1 {
      print!("{:02X} ", opcodes(1));
    } else {
      print!("   ")
    }

    if count > 2 {
      print!("{:02X} ", opcodes(2));
    } else {
      print!("   ")
    }

    print!("{:-}", opstring);

    let print_branch_target = || { print!("\t\t; ${:04x}", 0x5000 + pointer_counter + 2 + i64::try_from(opcodes(1)).unwrap()) };
    match opcodes(0) {
      0x10 | 0x30 | 0x50 | 0x70 | 0x90 | 0xB0 | 0xD0 | 0xF0 => print_branch_target(),
      _ => ()
    }

    print!("\n");
    count
  }
}
