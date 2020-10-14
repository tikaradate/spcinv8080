mod disassembler;
use crate::disassembler::disassembler8080;

fn main() {
    let binary = std::env::args().nth(1).expect("no binary given");
    let binary = std::fs::read(binary).expect("Unable to read file");

    disassembler8080(binary);
}
