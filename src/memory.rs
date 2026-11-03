use std::fs;
use std::io;

use crate::display::DebugDisplay;

// 4KB of RAM for the CPU
pub(crate) const MAX_MEM: usize = 0x1000;

// Programs are restricted from using the first 512 bytes of the memory space
pub(crate) const PROGRAM_START_OFFSET: usize = 0x200;

// The 16 hex digit sprites, 5 bytes each, start at the beginning of memory.
pub(crate) const FONT_SPRITE_OFFSET: usize = 0x00;

// Each hex digit is a 5-byte sprite.
pub(crate) const FONT_SPRITE_SIZE: usize = 5;

const FONT_SPRITES: [[u8; FONT_SPRITE_SIZE]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
    [0x20, 0x60, 0x20, 0x20, 0x70], // 1
    [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
    [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
    [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
    [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
    [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
    [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
    [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
    [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
    [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
    [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
    [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
    [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
    [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
    [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
];

#[derive(Debug)]
pub(crate) struct Memory {
    pub data: [u8; MAX_MEM],
}

impl Memory {
    pub fn initialise() -> Self {
        let mut memory = Memory { data: [0; MAX_MEM] };

        memory.setup_digit_sprites();

        memory
    }

    fn setup_digit_sprites(&mut self) {
        for (digit, sprite) in FONT_SPRITES.iter().enumerate() {
            let base = FONT_SPRITE_OFFSET + digit * FONT_SPRITE_SIZE;
            for (i, byte) in sprite.iter().enumerate() {
                self.data[base + i] = *byte;
            }
        }
    }

    pub fn initialise_from_file(file: &str) -> io::Result<Self> {
        let mut memory = Self::initialise();
        let rom = fs::read(file)?;

        for (i, byte) in rom.iter().enumerate() {
            let addr = PROGRAM_START_OFFSET + i;
            if addr >= MAX_MEM {
                break;
            }
            memory.data[addr] = *byte;
        }

        Ok(memory)
    }

    #[allow(dead_code)] // only used from within #[cfg(test)] modules
    pub fn insert_instruction(&mut self, index: usize, ins: u16) {
        self.data[index] = ((ins & 0xFF00) >> 8) as u8;
        self.data[index + 1] = (ins & 0x00FF) as u8;
    }

    pub fn write(&mut self, index: usize, val: u8) {
        if index >= MAX_MEM {
            panic!("Out of bounds memory write at address {:#04x}", index);
        }
        self.data[index] = val;
    }

    pub fn get(&self, index: usize) -> u8 {
        if index >= MAX_MEM {
            panic!("Out of bounds memory read at address {:#04x}", index);
        }

        self.data[index]
    }
}

impl DebugDisplay for Memory {
    fn view_state(&self) {
        let mut r = 0;
        print!("{:02x}: ", r);
        for i in 0..MAX_MEM {
            if i % 0x10 == 0 && i != 0 {
                r += 1;
                println!();
                print!("{:02x}: ", r);
            }

            print!("{:02x} ", self.data[i]);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fonts_loaded_compact() {
        let memory = Memory::initialise();

        // Digit 7 sprite lives at offset 7 * 5 = 35.
        assert_eq!(memory.data[35], 0xF0);
        assert_eq!(memory.data[39], 0x40);
        // Digit 0xF sprite lives at offset 15 * 5 = 75.
        assert_eq!(memory.data[75], 0xF0);
        assert_eq!(memory.data[79], 0x80);
    }

    #[test]
    #[should_panic(expected = "Out of bounds memory write")]
    fn memory_write_blocks_out_of_bounds() {
        let mut memory = Memory::initialise();
        memory.write(MAX_MEM, 0xFF);
    }

    #[test]
    #[should_panic(expected = "Out of bounds memory read")]
    fn memory_get_blocks_out_of_bounds() {
        let memory = Memory::initialise();
        memory.get(MAX_MEM);
    }

    #[test]
    fn rom_loading_stops_at_memory_end() {
        let dir = std::env::temp_dir();
        let path = dir.join("chip8_rs_huge_rom.bin");
        std::fs::write(&path, vec![0xAB; MAX_MEM + 100]).unwrap();

        let memory = Memory::initialise_from_file(path.to_str().unwrap()).unwrap();

        assert_eq!(memory.data[MAX_MEM - 1], 0xAB);
        std::fs::remove_file(path).unwrap();
    }
}
