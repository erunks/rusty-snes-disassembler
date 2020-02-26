use std::process;

mod opcode;

fn main() {
    if let Err(err) = opcode::handler::load_op_codes("./src/6502ops.csv") {
        println!("{}", err);
        process::exit(1);
    }
}
