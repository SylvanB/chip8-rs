use std::{cell::RefCell, fs::File, io::Write, rc::Rc};

use crate::{
    cpu::CPU,
    display::{Display, SCREEN_HEIGHT, SCREEN_WIDTH},
    keyboard::minifb_keyboard::MiniFbKeyboard,
    memory::Memory,
};
use clap::{load_yaml, App};
use minifb::{Key, Window, WindowOptions};

mod cpu;
mod display;
mod keyboard;
mod memory;
mod opcode;

// Chip-8 CPU based on Cowgod's Technical Spec for Chip-8
// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let window: Rc<RefCell<_>> = Rc::new(RefCell::new(
        Window::new(
            "Chip8.rs - ESC to exit - N to step, S to stop, C to continue",
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

    let memory = Memory::initialise_from_file(matches.value_of("INPUT").unwrap());
    let display = Display::initialise();
    let keyboard = MiniFbKeyboard::initialise(&window);
    let mut cpu = CPU::initialise(memory, display, keyboard);

    let mut inner_window = window.borrow_mut();
    inner_window.limit_update_rate(Some(std::time::Duration::from_micros(1000)));

    let last_cycle_time = chrono::Utc::now().time();
    let mut should_run = true;
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
        }

        if inner_window.is_key_pressed(Key::D, minifb::KeyRepeat::No) {
            println!("Dumping memory to chip8rs_memdump.log");
            dump_memory(&mut cpu.memory);
        }

        if should_run {
            cpu.execute_next_instruction();
        } else {
            if inner_window.is_key_pressed(Key::N, minifb::KeyRepeat::Yes) {
                cpu.execute_next_instruction();
            }
        }

        if inner_window.is_key_pressed(Key::S, minifb::KeyRepeat::No) {
            should_run = false;
        }

        if inner_window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
            should_run = true;
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
