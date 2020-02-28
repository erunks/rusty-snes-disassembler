use std::process;

mod opcode;
mod disassembler;

fn main() {
    let d = disassembler::Disassembler::new();
    // if let Err(err) = d::load_rom("../roms/Pieces.smc") {
    //     println!("{}", err);
    //     process::exit(1);
    // }
}
