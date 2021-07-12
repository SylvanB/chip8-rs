use std::mem;

use crate::{cpu::CPU, display::Display, memory::Memory};

mod cpu;
mod memory;
mod opcode;

// TODO: Implement these
mod display;
mod keyboard;

// Chip-8 CPU based on Cowgod's Technical Spec for Chip-8
// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
fn main() {
    println!("~ Iniitialising Chip-8 ~");

    let display = Display::initialise();
    let mut memory = Memory::initialise();

    memory.data[0x200] = 0x60;
    memory.data[0x201] = 0x08;

    memory.data[0x202] = 0x61;
    memory.data[0x203] = 0x0F;

    memory.data[0x204] = 0x62;
    memory.data[0x205] = 0x18;

    memory.data[0x206] = 0x63;
    memory.data[0x207] = 0x1F;

    let mut cpu = CPU::initialise(memory, display);

    cpu.execute();

    println!("Value of register V0: 0x{:x}", cpu.v[0]);
    println!("Value of register V1: 0x{:x}", cpu.v[1]);
    println!("Value of register V2: 0x{:x}", cpu.v[2]);
    println!("Value of register V3: 0x{:x}", cpu.v[3]);
}
