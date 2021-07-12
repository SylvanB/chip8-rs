#[derive(Debug)]
pub(crate) struct OpCode {
    inner: u16,
    id: u8,
    x: u8,
    y: u8,
    kk: u8,
    nnn: u16,
}

impl OpCode {
    pub fn new(raw_opcode: u16) -> Self {
        Self {
            inner: raw_opcode,
            id: ((raw_opcode & 0xF000) >> 12) as u8,
            x: ((raw_opcode & 0x0F00) >> 8) as u8,
            y: (raw_opcode & 0x00F0) as u8,
            kk: (raw_opcode & 0x00FF) as u8,
            nnn: (raw_opcode & 0x0FFF),
        }
    }

    pub fn raw(&self) -> u16 {
        self.inner
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn y(&self) -> u8 {
        self.y
    }

    pub fn kk(&self) -> u8 {
        self.kk
    }

    pub fn nnn(&self) -> u16 {
        self.nnn
    }
}
