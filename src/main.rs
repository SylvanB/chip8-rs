use crate::{
    cpu::CPU,
    display::{DebugDisplay, Display},
    memory::Memory,
};

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
    let memory = Memory::initialise();
    let mut cpu = CPU::initialise(memory, display);

    cpu.memory.insert_instruction(0x200, 0x6033);
    cpu.execute_next_instruction();
    cpu.view_state();

    cpu.memory.insert_instruction(0x202, 0x6144);
    cpu.execute_next_instruction();
    cpu.view_state();

    cpu.memory.insert_instruction(0x204, 0x6255);
    cpu.execute_next_instruction();
    cpu.view_state();

    cpu.memory.insert_instruction(0x206, 0x6366);
    cpu.execute_next_instruction();
    cpu.view_state();

    cpu.execute_next_instruction();
    cpu.view_state();

    // cpu.memory.view_state();

    cpu.display
        .display_sprite((&23, &4), &[0xF0, 0x80, 0xF0, 0x90, 0xF0]);

    cpu.display
        .display_sprite((&28, &4), &[0xF0, 0x90, 0xF0, 0x10, 0xF0]);

    cpu.display.display_sprite((&0, &0), &[0xFF]);

    cpu.display.view_state();

    // println!("Value of register V0: 0x{:x}", cpu.v[0]);
    // println!("Value of register V1: 0x{:x}", cpu.v[1]);
    // println!("Value of register V2: 0x{:x}", cpu.v[2]);
    // println!("Value of register V3: 0x{:x}", cpu.v[3]);
}
