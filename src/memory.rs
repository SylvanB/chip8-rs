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
