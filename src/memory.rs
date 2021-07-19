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
        Memory { data: [0; MAX_MEM] }
    }

    pub fn initialise_from_file(file: &str) -> Self {
        // TODO: Handle this nicely
        // let data = [0x0; MAX_MEM];
        let mut memory = Self {
            data: [0x0; MAX_MEM],
        };

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
