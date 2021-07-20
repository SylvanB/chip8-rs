use std::{cell::RefCell, fs::File, io::Write, rc::Rc};

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
            "Chip8.rs - ESC to exit - N to step through execution",
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

    // let memory = Memory::initialise_from_file("./roms/69.ch8");
    // let memory = Memory::initialise_from_file("./roms/test_opcode.ch8");
    let memory = Memory::initialise_from_file("./roms/BC_test.ch8");
    // let memory = Memory::initialise_from_file("./roms/stars.ch8");
    // let memory = Memory::initialise_from_file("./roms/life.ch8");

    // let memory = Memory::initialise();
    let display = Display::initialise();
    let keyboard = MiniFbKeyboard::initialise(&window);
    let mut cpu = CPU::initialise(memory, display, keyboard);

    // // Load the value 0x06 (the digit to display) into register V0
    // cpu.memory.insert_instruction(0x200, 0x6006);
    // // Load the value 0x00 into register V1 (x coord for the 6)
    // cpu.memory.insert_instruction(0x202, 0x6100);
    // // Get the memory location for the sprite for the character `6`
    // cpu.memory.insert_instruction(0x204, 0xF029);
    // // Display this sprite
    // cpu.memory.insert_instruction(0x206, 0xD115);

    // // Repeate the above for the character `9`

    // // Load the value 0x09 (the digit to display) into register V2
    // cpu.memory.insert_instruction(0x208, 0x6209);
    // // Load the value 0x05 into register V3 (x coord for the 9)
    // cpu.memory.insert_instruction(0x20A, 0x6305);

    // // Load the value 0x00 into register V4 (y coord for the 9)
    // cpu.memory.insert_instruction(0x20C, 0x6400);

    // // Get the memory location for the sprite for the character `9`
    // cpu.memory.insert_instruction(0x20E, 0xF229);
    // // Display this sprite
    // cpu.memory.insert_instruction(0x210, 0xD345);

    // Limit to max ~60 fps update rate
    let mut inner_window = window.borrow_mut();
    inner_window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let last_cycle_time = chrono::Utc::now().time();
    while inner_window.is_open() && !inner_window.is_key_down(Key::Escape) {
        let curr_cycle_time = chrono::Utc::now().time();

        if curr_cycle_time - last_cycle_time
            > chrono::Duration::milliseconds(cpu::DELAY_INCREMENT.into())
        {
            cpu.decrement_delay_timer();
        }

        if curr_cycle_time - last_cycle_time
            > chrono::Duration::milliseconds(cpu::SOUND_DELAY_INCREMENT.into())
        {
            cpu.decrement_sound_timer();
        }

        if inner_window.is_key_pressed(Key::N, minifb::KeyRepeat::Yes) {
            cpu.execute_next_instruction();
            // cpu.view_state();
        }

        if inner_window.is_key_pressed(Key::D, minifb::KeyRepeat::No) {
            println!("Dumping memory to chip8rs_memdump.log");
            dump_memory(&mut cpu.memory);
        }
        inner_window
            .update_with_buffer(&cpu.display.get_buffer(), SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}

fn dump_memory(memory: &mut Memory) {
    let mut file = File::create("chip8rs_memdump.log").unwrap();
    file.write_all(&memory.data).unwrap();
}
