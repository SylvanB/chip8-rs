// 4KB of RAM for the CPU
pub(crate) const MAX_MEM: usize = 4096;

// Programs are restricted from using the first 512 bytes of the memory space
pub(crate) const PROGRAM_START_OFFSET: usize = 0x200;

pub(crate) const ETI_600_PROGRAM_START_OFFSET: usize = 0x600;

#[derive(Debug)]
pub(crate) struct Memory {
    pub data: Vec<u8>,
}

impl Memory {
    pub fn initialise() -> Self {
        Memory {
            data: vec![0; MAX_MEM],
        }
    }

    pub fn get(&self, index: usize) -> u8 {
        // TODO: Remove this unwrap
        *self.data.get(index).unwrap()
    }
}
