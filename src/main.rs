use crate::{
    cpu::CPU,
    display::{DebugDisplay, Display, SCREEN_HEIGHT, SCREEN_WIDTH},
    memory::Memory,
};
use minifb::{Key, Window, WindowOptions};
use rand::Rng;

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

    let memory = Memory::initialise();
    let display = Display::initialise();
    let mut cpu = CPU::initialise(memory, display);

    // 6 sprite
    cpu.memory.data[0x600] = 0xF0;
    cpu.memory.data[0x601] = 0x80;
    cpu.memory.data[0x602] = 0xF0;
    cpu.memory.data[0x603] = 0x90;
    cpu.memory.data[0x604] = 0xF0;

    // 9 sprite
    cpu.memory.data[0x605] = 0xF0;
    cpu.memory.data[0x606] = 0x90;
    cpu.memory.data[0x607] = 0xF0;
    cpu.memory.data[0x608] = 0x10;
    cpu.memory.data[0x609] = 0xF0;

    cpu.memory.insert_instruction(0x200, 0xA600);
    cpu.memory.insert_instruction(0x202, 0xD255);
    cpu.memory.insert_instruction(0x204, 0xA605);
    cpu.memory.insert_instruction(0x206, 0xD755);
    cpu.memory.insert_instruction(0x208, 0x1200);

    let mut window = Window::new(
        "Chip8.rs - ESC to exit",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            scale: minifb::Scale::X8,
            scale_mode: minifb::ScaleMode::Stretch,
            topmost: false,
            transparency: false,
            none: false,
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        cpu.execute_next_instruction();

        window
            .update_with_buffer(&cpu.display.get_buffer(), SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}
