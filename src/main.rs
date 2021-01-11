use std::process;

mod opcode;
mod disassembler;

fn get_input() -> String {
    use std::io;
    println!("Enter path to ROM file:");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_) => {},
    }
    input.trim().to_string()
}

fn main() {
    let mut d = disassembler::Disassembler::new();
    d.load_rom(&get_input());
    if let Err(err) = d.disassemble_rom() {
        println!("{}", err);
        process::exit(1);
    }
}
