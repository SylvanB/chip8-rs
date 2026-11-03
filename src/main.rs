use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};

use clap::Parser;
use minifb::{Key, KeyRepeat, Scale, ScaleMode, Window, WindowOptions};

use crate::{
    cpu::CPU,
    debug_gui::{DebugWindow, DEBUG_HEIGHT, DEBUG_WIDTH},
    display::{Display, SCREEN_HEIGHT, SCREEN_WIDTH},
    keyboard::{minifb_keyboard::MiniFbKeyboard, Keyboard},
    memory::Memory,
};

mod cpu;
mod debug_gui;
mod disasm;
mod display;
mod keyboard;
mod memory;
mod opcode;

// The delay/sound timers tick at 60 Hz.
const TIMER_TICK: Duration = Duration::from_micros(16_667);

// A typical CHIP-8 runs a few hundred instructions per second.
const DEFAULT_SPEED: u64 = 700;

const SCALE_OPTIONS: &[usize] = &[1, 2, 4, 8, 16, 32];

#[derive(Parser)]
#[command(about = "A Chip-8 interpreter")]
struct Args {
    /// The Chip8 ROM (.ch8) file to use
    input: String,

    /// CPU speed, in instructions executed per second
    #[arg(short, long, default_value_t = DEFAULT_SPEED)]
    speed: u64,

    /// Window scale factor (1, 2, 4, 8, 16 or 32)
    #[arg(long, default_value_t = 8)]
    scale: usize,

    /// Open a separate debugger window showing CPU state, disassembly and memory
    #[arg(long)]
    debug: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let scale = minifb_scale(args.scale).ok_or_else(|| {
        format!(
            "{} is not a valid scale; choose one of {:?}",
            args.scale, SCALE_OPTIONS
        )
    })?;

    let mut window = Window::new(
        "Chip8.rs - ESC: quit, F1: mem dump, F2: step, F3: pause, F4: resume",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions {
            scale,
            scale_mode: ScaleMode::Stretch,
            ..WindowOptions::default()
        },
    )?;
    window.limit_update_rate(Some(Duration::from_millis(16)));

    let memory = Memory::initialise_from_file(&args.input)?;
    let display = Display::initialise();
    let keyboard = MiniFbKeyboard::initialise(&window);
    let mut cpu = CPU::initialise(memory, display, keyboard);

    let mut debugger: Option<Window> = if args.debug {
        Some(Window::new(
            "Chip8.rs - Debugger",
            DEBUG_WIDTH,
            DEBUG_HEIGHT,
            WindowOptions {
                scale: Scale::X2,
                scale_mode: ScaleMode::Stretch,
                ..WindowOptions::default()
            },
        )?)
    } else {
        None
    };
    let mut debug_window = debugger
        .as_ref()
        .map(|_| DebugWindow::initialise(DEBUG_WIDTH, DEBUG_HEIGHT));

    let mut running = true;
    let mut last_frame = Instant::now();
    let mut timer_accumulator = Duration::ZERO;
    let mut cycle_accumulator = 0.0;
    let mut was_beeping = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        let elapsed = now - last_frame;
        last_frame = now;

        // Tick the delay/sound timers at 60 Hz regardless of frame rate.
        timer_accumulator += elapsed;
        while timer_accumulator >= TIMER_TICK {
            cpu.decrement_delay_timer();
            cpu.decrement_sound_timer();
            timer_accumulator -= TIMER_TICK;
        }

        // Beep once each time the sound timer activates.
        if cpu.sound_timer > 0 && !was_beeping {
            play_beep();
            was_beeping = true;
        } else if cpu.sound_timer == 0 {
            was_beeping = false;
        }

        let keys: Vec<u8> = window
            .get_keys_pressed(KeyRepeat::Yes)
            .unwrap_or_default()
            .iter()
            .filter_map(|k| key_to_u8(*k))
            .collect();
        cpu.keyboard.update_state(&keys);

        if running {
            if cpu.is_waiting_for_key() {
                // FX0A has halted the CPU; resume the moment a key is pressed.
                if !keys.is_empty() {
                    cpu.execute_next_instruction();
                }
                cycle_accumulator = 0.0;
            } else {
                // Execute as many instructions as the elapsed time permits.
                cycle_accumulator += elapsed.as_secs_f64() * args.speed as f64;
                let mut steps = cycle_accumulator as usize;
                cycle_accumulator -= steps as f64;

                while steps > 0 {
                    if cpu.is_waiting_for_key() {
                        break;
                    }
                    cpu.execute_next_instruction();
                    steps -= 1;
                }
            }
        } else if window.is_key_pressed(Key::F2, KeyRepeat::Yes) {
            cpu.execute_next_instruction();
        }

        if window.is_key_pressed(Key::F3, KeyRepeat::No) {
            running = false;
        }
        if window.is_key_pressed(Key::F4, KeyRepeat::No) {
            running = true;
        }
        if window.is_key_pressed(Key::F1, KeyRepeat::No) {
            println!("Dumping memory to chip8rs_memdump.log");
            dump_memory(&cpu.memory);
        }

        window.update_with_buffer(&cpu.display.get_buffer(), SCREEN_WIDTH, SCREEN_HEIGHT)?;

        if let (Some(debug_window), Some(debugger)) = (debug_window.as_mut(), debugger.as_mut()) {
            if debugger.is_open() {
                debug_window.render(&cpu, running);
                debugger.update_with_buffer(debug_window.buffer(), DEBUG_WIDTH, DEBUG_HEIGHT)?;
            }
        }
    }

    Ok(())
}

fn minifb_scale(scale: usize) -> Option<Scale> {
    match scale {
        1 => Some(Scale::X1),
        2 => Some(Scale::X2),
        4 => Some(Scale::X4),
        8 => Some(Scale::X8),
        16 => Some(Scale::X16),
        32 => Some(Scale::X32),
        _ => None,
    }
}

fn key_to_u8(key: Key) -> Option<u8> {
    match key {
        Key::Key0 => Some(0x0),
        Key::Key1 => Some(0x1),
        Key::Key2 => Some(0x2),
        Key::Key3 => Some(0x3),
        Key::Key4 => Some(0x4),
        Key::Key5 => Some(0x5),
        Key::Key6 => Some(0x6),
        Key::Key7 => Some(0x7),
        Key::Key8 => Some(0x8),
        Key::Key9 => Some(0x9),
        Key::A => Some(0xA),
        Key::B => Some(0xB),
        Key::C => Some(0xC),
        Key::D => Some(0xD),
        Key::E => Some(0xE),
        Key::F => Some(0xF),
        _ => None,
    }
}

fn dump_memory(memory: &Memory) {
    let mut file = File::create("chip8rs_memdump.log").expect("Failed to open memory dump file");
    file.write_all(&memory.data)
        .expect("Failed to write memory dump file");
}

fn play_beep() {
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("afplay")
        .arg("/System/Library/Sounds/Glass.aiff")
        .spawn();

    #[cfg(target_os = "linux")]
    let _ = std::process::Command::new("paplay")
        .arg("/usr/share/sounds/freedesktop/stereo/bell.oga")
        .spawn();

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    println!("\x07");
}
