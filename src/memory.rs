use std::fs;

use crate::display::DebugDisplay;

// 4KB of RAM for the CPU
pub(crate) const MAX_MEM: usize = 0x1000;

// Programs are restricted from using the first 512 bytes of the memory space
pub(crate) const PROGRAM_START_OFFSET: usize = 0x200;

pub(crate) const ETI_600_PROGRAM_START_OFFSET: usize = 0x600;

#[derive(Debug)]
pub(crate) struct Memory {
    pub data: [u8; MAX_MEM],
}

impl Memory {
    pub fn initialise() -> Self {
        let memory = Memory { data: [0; MAX_MEM] };

        memory
    }

    fn setup_digit_sprites(&mut self) {
        // Yes we are wasting alot of bytes here.
        // No I dont care.
        // No I wont fix it.
        // its for my own sanity of remembering where the hell the sprites are.

        // 0
        self.data[0x0] = 0xF0;
        self.data[0x1] = 0x90;
        self.data[0x2] = 0x90;
        self.data[0x3] = 0x90;
        self.data[0x4] = 0xF0;

        // 1
        self.data[0x10] = 0x20;
        self.data[0x11] = 0x60;
        self.data[0x12] = 0x20;
        self.data[0x13] = 0x20;
        self.data[0x14] = 0x70;

        // 2
        self.data[0x20] = 0xF0;
        self.data[0x21] = 0x10;
        self.data[0x22] = 0xF0;
        self.data[0x23] = 0x80;
        self.data[0x24] = 0xF0;

        // 3
        self.data[0x30] = 0xF0;
        self.data[0x31] = 0x10;
        self.data[0x32] = 0xF0;
        self.data[0x33] = 0x10;
        self.data[0x34] = 0xF0;

        // 4
        self.data[0x40] = 0x90;
        self.data[0x41] = 0x90;
        self.data[0x42] = 0xF0;
        self.data[0x43] = 0x10;
        self.data[0x44] = 0x10;

        // 5
        self.data[0x50] = 0xF0;
        self.data[0x51] = 0x80;
        self.data[0x52] = 0xF0;
        self.data[0x53] = 0x10;
        self.data[0x54] = 0xF0;

        // 6
        self.data[0x60] = 0xF0;
        self.data[0x61] = 0x80;
        self.data[0x62] = 0xF0;
        self.data[0x63] = 0x90;
        self.data[0x64] = 0xF0;

        // 7
        self.data[0x70] = 0xF0;
        self.data[0x71] = 0x10;
        self.data[0x72] = 0x20;
        self.data[0x73] = 0x40;
        self.data[0x74] = 0x40;

        // 8
        self.data[0x80] = 0xF0;
        self.data[0x81] = 0x90;
        self.data[0x82] = 0xF0;
        self.data[0x83] = 0x90;
        self.data[0x84] = 0xF0;

        // 9
        self.data[0x90] = 0xF0;
        self.data[0x91] = 0x90;
        self.data[0x92] = 0xF0;
        self.data[0x93] = 0x10;
        self.data[0x94] = 0xF0;

        // 10 / 0xA
        self.data[0xA0] = 0xF0;
        self.data[0xA1] = 0x90;
        self.data[0xA2] = 0xF0;
        self.data[0xA3] = 0x90;
        self.data[0xA4] = 0x90;

        // 11 / 0xA
        self.data[0xB0] = 0xE0;
        self.data[0xB1] = 0x90;
        self.data[0xB2] = 0xE0;
        self.data[0xB3] = 0x90;
        self.data[0xB4] = 0xE0;

        // 12 / 0xC
        self.data[0xC0] = 0xF0;
        self.data[0xC1] = 0x80;
        self.data[0xC2] = 0x80;
        self.data[0xC3] = 0x80;
        self.data[0xC4] = 0xF0;

        // 13 / 0xD
        self.data[0xD0] = 0xE0;
        self.data[0xD1] = 0x90;
        self.data[0xD2] = 0x90;
        self.data[0xD3] = 0x90;
        self.data[0xD4] = 0xE0;

        // 14 / 0xE
        self.data[0xE0] = 0xF0;
        self.data[0xE1] = 0x80;
        self.data[0xE2] = 0xF0;
        self.data[0xE3] = 0x80;
        self.data[0xE4] = 0xF0;

        // 15 / 0xF
        self.data[0xF0] = 0xF0;
        self.data[0xF1] = 0x80;
        self.data[0xF2] = 0xF0;
        self.data[0xF3] = 0x80;
        self.data[0xF4] = 0x80;
    }

    pub fn initialise_from_file(file: &str) -> Self {
        // TODO: Handle this nicely
        let mut memory = Self::initialise();

        let file = fs::read(file).unwrap();

        let mut i = 0x200;
        for b in file {
            if i > MAX_MEM {
                break;
            }
            memory.data[i] = b;
            i += 1;
        }

        memory
    }

    pub fn insert_instruction(&mut self, index: usize, ins: u16) {
        // TODO: Ensure that ops only start at even addresses (see spec line 193)

        self.data[index] = ((ins & 0xFF00) >> 8) as u8;
        self.data[index + 1] = (ins & 0x00FF) as u8;
    }

    pub fn write(&mut self, index: usize, val: u8) {
        self.data[index] = val;
    }

    pub fn get(&self, index: usize) -> u8 {
        // TODO: Support ETI_600 offset
        // TODO: Handle this error better - maybe return some Result variant.
        if index > MAX_MEM || index < PROGRAM_START_OFFSET {
            panic!("Out of bound memory expcetion");
        }

        // TODO: Remove this unwrap
        *self.data.get(index).unwrap()
    }
}

impl DebugDisplay for Memory {
    fn view_state(&self) {
        let mut r = 0;
        print!("{:02x}: ", r);
        for i in 0..MAX_MEM {
            if i % 0x10 == 0 && i != 0 {
                r += 1;
                println!("");
                print!("{:02x}: ", r);
            }

            print!("{:02x} ", self.data[i]);
        }
        println!("");
    }
}
