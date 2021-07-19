use std::{cell::RefCell, rc::Rc};

use crate::{
    cpu::CPU,
    display::{DebugDisplay, Display, SCREEN_HEIGHT, SCREEN_WIDTH},
    keyboard::minifb_keyboard::MiniFbKeyboard,
    memory::Memory,
};
use minifb::{Key, Window, WindowOptions};

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

    let window: Rc<RefCell<_>> = Rc::new(RefCell::new(
        Window::new(
            "Chip8.rs - ESC to exit",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            WindowOptions {
                scale: minifb::Scale::X8,
                scale_mode: minifb::ScaleMode::Stretch,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        }),
    ));

    // let memory = Memory::initialise_from_file("./roms/test_opcode.ch8");
    let memory = Memory::initialise();
    let display = Display::initialise();
    let keyboard = MiniFbKeyboard::initialise(&window);
    let mut cpu = CPU::initialise(memory, display, keyboard);

    // Load the value 0x06 into register V0
    cpu.memory.insert_instruction(0x200, 0x6006);
    // Get the memory location for the sprite for the character `6`
    cpu.memory.insert_instruction(0x202, 0xF029);
    // Display this sprite
    cpu.memory.insert_instruction(0x204, 0xD005);

    // Repeate the above for the character `9`
    cpu.memory.insert_instruction(0x206, 0x6109);
    cpu.memory.insert_instruction(0x208, 0xF129);
    cpu.memory.insert_instruction(0x20A, 0xD505);

    // Limit to max ~60 fps update rate
    let mut inner_window = window.borrow_mut();
    inner_window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while inner_window.is_open() && !inner_window.is_key_down(Key::Escape) {
        cpu.execute_next_instruction();
        // cpu.view_state();
        inner_window
            .update_with_buffer(&cpu.display.get_buffer(), SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}
