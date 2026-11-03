// Disassembler that turns raw opcodes into CHIP-8 mnemonics for the debug view.

const HEX: &[u8] = b"0123456789ABCDEF";

fn reg(x: u8) -> String {
    format!("V{}", HEX[(x & 0xF) as usize] as char)
}

/// Decodes a single 16-bit opcode into a human-readable mnemonic.
pub fn disassemble(opcode: u16) -> String {
    let x = ((opcode >> 8) & 0xF) as u8;
    let y = ((opcode >> 4) & 0xF) as u8;
    let n = (opcode & 0xF) as u8;
    let kk = (opcode & 0xFF) as u8;
    let nnn = opcode & 0x0FFF;

    match opcode >> 12 {
        0x0 => match opcode {
            0x00E0 => "CLS".to_string(),
            0x00EE => "RET".to_string(),
            _ => format!("SYS {:03X}", nnn),
        },
        0x1 => format!("JP {:03X}", nnn),
        0x2 => format!("CALL {:03X}", nnn),
        0x3 => format!("SE {}, {:02X}", reg(x), kk),
        0x4 => format!("SNE {}, {:02X}", reg(x), kk),
        0x5 => {
            if opcode & 0xF == 0 {
                format!("SE {}, {}", reg(x), reg(y))
            } else {
                "???".to_string()
            }
        }
        0x6 => format!("LD {}, {:02X}", reg(x), kk),
        0x7 => format!("ADD {}, {:02X}", reg(x), kk),
        0x8 => match opcode & 0xF {
            0x0 => format!("LD {}, {}", reg(x), reg(y)),
            0x1 => format!("OR {}, {}", reg(x), reg(y)),
            0x2 => format!("AND {}, {}", reg(x), reg(y)),
            0x3 => format!("XOR {}, {}", reg(x), reg(y)),
            0x4 => format!("ADD {}, {}", reg(x), reg(y)),
            0x5 => format!("SUB {}, {}", reg(x), reg(y)),
            0x6 => format!("SHR {}", reg(x)),
            0x7 => format!("SUBN {}, {}", reg(x), reg(y)),
            0xE => format!("SHL {}", reg(x)),
            _ => "???".to_string(),
        },
        0x9 => {
            if opcode & 0xF == 0 {
                format!("SNE {}, {}", reg(x), reg(y))
            } else {
                "???".to_string()
            }
        }
        0xA => format!("LD I, {:03X}", nnn),
        0xB => format!("JP V0, {:03X}", nnn),
        0xC => format!("RND {}, {:02X}", reg(x), kk),
        0xD => format!("DRW {}, {}, {}", reg(x), reg(y), n),
        0xE => match opcode & 0xFF {
            0x9E => format!("SKP {}", reg(x)),
            0xA1 => format!("SKNP {}", reg(x)),
            _ => "???".to_string(),
        },
        0xF => match opcode & 0xFF {
            0x07 => format!("LD {}, DT", reg(x)),
            0x0A => format!("LD {}, K", reg(x)),
            0x15 => format!("LD DT, {}", reg(x)),
            0x18 => format!("LD ST, {}", reg(x)),
            0x1E => format!("ADD I, {}", reg(x)),
            0x29 => format!("LD F, {}", reg(x)),
            0x33 => format!("LD B, {}", reg(x)),
            0x55 => format!("LD [I], {}", reg(x)),
            0x65 => format!("LD {}, [I]", reg(x)),
            _ => "???".to_string(),
        },
        _ => "???".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::disassemble;

    #[test]
    fn zero_family() {
        assert_eq!(disassemble(0x00E0), "CLS");
        assert_eq!(disassemble(0x00EE), "RET");
        assert_eq!(disassemble(0x0210), "SYS 210");
    }

    #[test]
    fn jump_call() {
        assert_eq!(disassemble(0x1ABC), "JP ABC");
        assert_eq!(disassemble(0x2DEF), "CALL DEF");
    }

    #[test]
    fn skip_and_load_immediate() {
        assert_eq!(disassemble(0x3A12), "SE VA, 12");
        assert_eq!(disassemble(0x4A12), "SNE VA, 12");
        assert_eq!(disassemble(0x5AB0), "SE VA, VB");
        assert_eq!(disassemble(0x6A12), "LD VA, 12");
        assert_eq!(disassemble(0x7A12), "ADD VA, 12");
    }

    #[test]
    fn register_ops() {
        assert_eq!(disassemble(0x8AB0), "LD VA, VB");
        assert_eq!(disassemble(0x8AB1), "OR VA, VB");
        assert_eq!(disassemble(0x8AB2), "AND VA, VB");
        assert_eq!(disassemble(0x8AB3), "XOR VA, VB");
        assert_eq!(disassemble(0x8AB4), "ADD VA, VB");
        assert_eq!(disassemble(0x8AB5), "SUB VA, VB");
        assert_eq!(disassemble(0x8AB6), "SHR VA");
        assert_eq!(disassemble(0x8AB7), "SUBN VA, VB");
        assert_eq!(disassemble(0x8ABE), "SHL VA");
    }

    #[test]
    fn remaining_families() {
        assert_eq!(disassemble(0x9AB0), "SNE VA, VB");
        assert_eq!(disassemble(0xAFFF), "LD I, FFF");
        assert_eq!(disassemble(0xB123), "JP V0, 123");
        assert_eq!(disassemble(0xC123), "RND V1, 23");
        assert_eq!(disassemble(0xD123), "DRW V1, V2, 3");
        assert_eq!(disassemble(0xE29E), "SKP V2");
        assert_eq!(disassemble(0xE2A1), "SKNP V2");
        assert_eq!(disassemble(0xF007), "LD V0, DT");
        assert_eq!(disassemble(0xF00A), "LD V0, K");
        assert_eq!(disassemble(0xF015), "LD DT, V0");
        assert_eq!(disassemble(0xF018), "LD ST, V0");
        assert_eq!(disassemble(0xF01E), "ADD I, V0");
        assert_eq!(disassemble(0xF029), "LD F, V0");
        assert_eq!(disassemble(0xF033), "LD B, V0");
        assert_eq!(disassemble(0xF055), "LD [I], V0");
        assert_eq!(disassemble(0xF065), "LD V0, [I]");
    }

    #[test]
    fn invalid_ops() {
        assert_eq!(disassemble(0x0000), "SYS 000");
        assert_eq!(disassemble(0xFFFF), "???");
    }
}
