#[derive(Debug)]
pub struct OpCode {
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
            y: ((raw_opcode & 0x00F0) >> 4) as u8,
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

#[cfg(test)]
mod tests {
    use crate::opcode::OpCode;

    #[test]
    fn should_generate_correct_raw_value() {
        assert_eq!(OpCode::new(0x1234).raw(), 0x1234);
    }

    #[test]
    fn should_generate_correct_id_value() {
        assert_eq!(OpCode::new(0x1234).id(), 0x1);
    }

    #[test]
    fn should_generate_correct_x_value() {
        assert_eq!(OpCode::new(0x1234).x(), 0x2);
    }

    #[test]
    fn should_generate_correct_y_value() {
        assert_eq!(OpCode::new(0x1234).y(), 0x3);
    }

    #[test]
    fn should_generate_correct_kk_value() {
        assert_eq!(OpCode::new(0x1234).kk(), 0x34);
    }

    #[test]
    fn should_generate_correct_nnn_value() {
        assert_eq!(OpCode::new(0x1234).nnn(), 0x234);
    }
}
