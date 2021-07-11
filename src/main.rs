use std::mem;

use crate::{cpu::CPU, display::Display, memory::Memory};

mod cpu;
mod memory;

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
    memory.data[0x201] = 0xFF;

    let mut cpu = CPU::initialise(memory, display);

    cpu.execute();

    println!("Value of register V0: {}", cpu.v[0]);
}
