mod p8080;
mod disassembler;
use crate::disassembler::disassembler8080;

fn main() {
    // make it optional?
    let binary = std::env::args().nth(1).expect("no binary given");
    let binary = std::fs::read(binary).expect("unable to read file");

    disassembler8080(binary);
}
