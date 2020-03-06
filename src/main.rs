use std::process;

mod opcode;
mod disassembler;

fn main() {
    let mut d = disassembler::Disassembler::new();
    d.load_rom("./roms/Pieces.smc");
    if let Err(err) = d.disassemble_rom() {
        println!("{}", err);
        process::exit(1);
    }
}
