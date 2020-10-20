struct Flags {
    // if result is zero
    z: bool,
    // if result is signed
    s: bool,
    // if the number of 1's is even
    p: bool,
    // if needs carry
    cy: bool,
    // if needs auxiliar carry, used in BCD
    ac: bool,
}

pub struct Processor {
    flags: Flags,
    // the 7 registers
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    // stack pointer
    sp: u16,
    // program counter
    pc: u16,
    // 64kb memory
    memory: [u8; 65536],
    int_enable: u8,
}

impl Processor {

}