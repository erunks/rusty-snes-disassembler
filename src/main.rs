use std::process;

mod opCodeHandler;

fn main() {
    if let Err(err) = opCodeHandler::load_op_codes("./src/6502ops.csv") {
        println!("{}", err);
        process::exit(1);
    }
}
