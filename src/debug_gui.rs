// On-screen debugger: renders the CPU's internal state to a second window
// using a 5x7 bitmap font. No external text rendering is available through
// minifb, so we bring our own tiny font.

use crate::{
    cpu::CPU,
    disasm::disassemble,
    keyboard::Keyboard,
};

pub const DEBUG_WIDTH: usize = 200;
pub const DEBUG_HEIGHT: usize = 176;

const GLYPH_W: usize = 5;
const PITCH_X: usize = 6;
const PITCH_Y: usize = 8;

const MAX_ROWS: usize = DEBUG_HEIGHT / PITCH_Y; // 22

const COLOR_BG: u32 = 0xFF_00_00_00;
const COLOR_TEXT: u32 = 0xFF_E8_E8_E8;
const COLOR_DIM: u32 = 0xFF_80_80_80;
const COLOR_ACCENT: u32 = 0xFF_3A_E0_6C;
const COLOR_WARN: u32 = 0xFF_FF_DD_55;

pub struct DebugWindow {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl DebugWindow {
    pub fn initialise(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![COLOR_BG; width * height],
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(COLOR_BG);
    }

    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }

    fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    /// Draws a single character with its top-left corner at (x, y).
    /// Lowercase letters are drawn as their uppercase counterpart.
    fn draw_char(&mut self, x: usize, y: usize, ch: char, color: u32) {
        let slot = ch.to_ascii_uppercase() as usize;
        if !(0x20..0x60).contains(&slot) {
            return;
        }

        let glyph = FONT_5X7[slot - 0x20];
        for (row, mask) in glyph.iter().enumerate() {
            for col in 0..GLYPH_W {
                if mask & (0x10 >> col) != 0 {
                    self.put_pixel(x + col, y + row, color);
                }
            }
        }
    }

    fn draw_text(&mut self, x: usize, y: usize, text: &str, color: u32) {
        for (i, ch) in text.chars().enumerate() {
            self.draw_char(x + i * PITCH_X, y, ch, color);
        }
    }

    /// Convenience: writes a text line at a fixed row (0-based).
    fn line(&mut self, row: usize, text: &str, color: u32) {
        if row < MAX_ROWS {
            self.draw_text(2, row * PITCH_Y, text, color);
        }
    }

    /// Renders the complete debugger view for the given CPU state.
    pub fn render<T: Keyboard>(&mut self, cpu: &CPU<T>, running: bool) {
        self.clear();

        self.line(0, "CHIP-8 DEBUGGER", COLOR_ACCENT);
        let status = if cpu.is_waiting_for_key() {
            "WAITKEY"
        } else if running {
            "RUN"
        } else {
            "PAUSED"
        };
        let status_color = if running { COLOR_DIM } else { COLOR_WARN };
        self.draw_text(
            DEBUG_WIDTH - 2 - status.len() * PITCH_X,
            0,
            status,
            status_color,
        );

        let state = format!(
            "PC={:03X} I={:03X} SP={:02X} DT={:02X} ST={:02X}",
            cpu.pc, cpu.vi, cpu.sp, cpu.delay_timer, cpu.sound_timer
        );
        self.line(1, &state, COLOR_TEXT);

        // Registers, four per row; flag register gets emphasised.
        for (row, chunk) in cpu.v.chunks(4).enumerate() {
            let mut parts = Vec::with_capacity(4);
            for (i, value) in chunk.iter().enumerate() {
                let idx = row as u8 * 4 + i as u8;
                parts.push(format!("{}={:02X}", reg_name(idx), value));
            }
            let color = if row == 3 { COLOR_ACCENT } else { COLOR_TEXT };
            self.line(3 + row, &parts.join("  "), color);
        }

        // Stack, most recent entry first.
        self.line(7, "STACK TOP->BOTTOM", COLOR_DIM);
        let mut stack_text = String::new();
        for i in (0..=cpu.sp as usize).rev().take(4) {
            if !stack_text.is_empty() {
                stack_text.push(' ');
            }
            stack_text.push_str(&format!("{:02X}:{:04X}", i, cpu.stack[i]));
        }
        self.line(8, &stack_text, COLOR_TEXT);

        // Disassembly around the PC.
        self.line(10, "DISASM @PC", COLOR_DIM);
        for offset in 0..4 {
            let addr = cpu.pc.wrapping_add(offset * 2);
            match cpu.peek_word(addr) {
                Some(opcode) => {
                    let text = format!("{:03X} {:04X}  {}", addr, opcode, disassemble(opcode));
                    let color = if offset == 0 { COLOR_ACCENT } else { COLOR_TEXT };
                    self.line(11 + offset as usize, &text, color);
                }
                None => break,
            }
        }

        // Memory dump at the I register and at the PC.
        self.line(16, &format!("MEM@I   {:03X}", cpu.vi & 0x0FFF), COLOR_DIM);
        self.memory_lines(cpu, cpu.vi as usize & 0x0FFF, 17);

        self.line(19, &format!("MEM@PC  {:03X}", cpu.pc & 0x0FFF), COLOR_DIM);
        self.memory_lines(cpu, cpu.pc as usize, 20);
    }

    fn memory_lines<T: Keyboard>(&mut self, cpu: &CPU<T>, base: usize, start_row: usize) {
        for row in 0..2 {
            let addr = base + row * 8;
            let mut text = format!("{:03X}", addr & 0x0FFF);
            for byte_idx in 0..8 {
                let idx = addr + byte_idx;
                let byte = if idx < crate::memory::MAX_MEM {
                    cpu.memory.data[idx]
                } else {
                    0
                };
                text.push_str(&format!(" {:02X}", byte));
            }
            self.line(start_row + row, &text, COLOR_TEXT);
        }
    }
}

fn reg_name(x: u8) -> String {
    format!("V{:X}", x)
}

// 5x7 bitmap font for characters 0x20..0x5F (space through '_').
// Each glyph is 7 rows of 5 bits; bit 4 is the leftmost column.
const FONT_5X7: [[u8; 7]; 0x60 - 0x20] = [
    // space
    [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
    // !
    [0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00000, 0b00100],
    // "
    [0b01010, 0b01010, 0b01010, 0b00000, 0b00000, 0b00000, 0b00000],
    // #
    [0b01010, 0b11111, 0b01010, 0b11111, 0b01010, 0b00000, 0b00000],
    // $
    [0b00100, 0b01110, 0b10001, 0b01110, 0b10001, 0b01110, 0b00100],
    // %
    [0b11000, 0b11000, 0b00100, 0b00010, 0b00001, 0b00011, 0b00011],
    // &
    [0b01110, 0b10001, 0b10001, 0b01110, 0b10101, 0b10010, 0b01101],
    // '
    [0b00100, 0b00100, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
    // (
    [0b00100, 0b01000, 0b10000, 0b10000, 0b10000, 0b01000, 0b00100],
    // )
    [0b00100, 0b00010, 0b00001, 0b00001, 0b00001, 0b00010, 0b00100],
    // *
    [0b00000, 0b10101, 0b01110, 0b11111, 0b01110, 0b10101, 0b00000],
    // +
    [0b00000, 0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0b00000],
    // ,
    [0b00000, 0b00000, 0b00000, 0b00000, 0b00100, 0b00100, 0b01000],
    // -
    [0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000],
    // .
    [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00110, 0b00110],
    // /
    [0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b10000, 0b10000],
    // 0
    [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
    // 1
    [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
    // 2
    [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111],
    // 3
    [0b01110, 0b10001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110],
    // 4
    [0b10001, 0b10011, 0b10101, 0b11001, 0b11111, 0b10001, 0b10001],
    // 5
    [0b11111, 0b10000, 0b10000, 0b11110, 0b00001, 0b10001, 0b01110],
    // 6
    [0b01110, 0b10001, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110],
    // 7
    [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000],
    // 8
    [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110],
    // 9
    [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100],
    // :
    [0b00000, 0b00110, 0b00110, 0b00000, 0b00110, 0b00110, 0b00000],
    // ;
    [0b00000, 0b00110, 0b00110, 0b00000, 0b00110, 0b00100, 0b01000],
    // <
    [0b00010, 0b00100, 0b01000, 0b10000, 0b01000, 0b00100, 0b00010],
    // =
    [0b00000, 0b00000, 0b11111, 0b00000, 0b11111, 0b00000, 0b00000],
    // >
    [0b01000, 0b00100, 0b00010, 0b00001, 0b00010, 0b00100, 0b01000],
    // ?
    [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b00000, 0b00100],
    // @
    [0b01110, 0b10001, 0b00001, 0b01101, 0b10101, 0b10001, 0b01110],
    // A
    [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
    // B
    [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110],
    // C
    [0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110],
    // D
    [0b11100, 0b10010, 0b10001, 0b10001, 0b10001, 0b10010, 0b11100],
    // E
    [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
    // F
    [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000],
    // G
    [0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01111],
    // H
    [0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
    // I
    [0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
    // J
    [0b00111, 0b00010, 0b00010, 0b00010, 0b00010, 0b10010, 0b01100],
    // K
    [0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001],
    // L
    [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
    // M
    [0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001],
    // N
    [0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001],
    // O
    [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
    // P
    [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000],
    // Q
    [0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101],
    // R
    [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001],
    // S
    [0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110],
    // T
    [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
    // U
    [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
    // V
    [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
    // W
    [0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010],
    // X
    [0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001],
    // Y
    [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100],
    // Z
    [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111],
    // [
    [0b01110, 0b01000, 0b01000, 0b01000, 0b01000, 0b01000, 0b01110],
    // backslash
    [0b10000, 0b10000, 0b01000, 0b00100, 0b00010, 0b00001, 0b00001],
    // ]
    [0b01110, 0b00010, 0b00010, 0b00010, 0b00010, 0b00010, 0b01110],
    // ^
    [0b00100, 0b01010, 0b10001, 0b00000, 0b00000, 0b00000, 0b00000],
    // _
    [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111],
];

#[cfg(test)]
mod tests {
    use crate::{
        cpu::CPU,
        display::Display,
        keyboard::dummy_keyboard::DummyKeyboard,
        memory::Memory,
    };

    use super::*;

    fn cpu() -> CPU<DummyKeyboard> {
        CPU::initialise(
            Memory::initialise(),
            Display::initialise(),
            DummyKeyboard::initialise(),
        )
    }

    #[test]
    fn render_produces_content() {
        let mut panel = DebugWindow::initialise(DEBUG_WIDTH, DEBUG_HEIGHT);
        let cpu = cpu();

        panel.render(&cpu, true);

        let on_pixels = panel.buffer().iter().filter(|&&p| p == COLOR_TEXT).count();
        assert!(on_pixels > 0);
    }

    #[test]
    fn draw_char_sets_glyph_pixels() {
        let mut panel = DebugWindow::initialise(DEBUG_WIDTH, DEBUG_HEIGHT);

        panel.draw_char(0, 0, 'A', COLOR_TEXT);

        // The 'A' glyph's top row is 0b01110 -> pixels at columns 1, 2 and 3.
        assert_eq!(panel.buffer()[0], COLOR_BG);
        assert_eq!(panel.buffer()[1], COLOR_TEXT);
        assert_eq!(panel.buffer()[2], COLOR_TEXT);
        assert_eq!(panel.buffer()[3], COLOR_TEXT);
        assert_eq!(panel.buffer()[4], COLOR_BG);
    }

    #[test]
    fn draw_char_out_of_range_is_ignored() {
        let mut panel = DebugWindow::initialise(DEBUG_WIDTH, DEBUG_HEIGHT);

        panel.draw_char(0, 0, 'a', COLOR_TEXT); // lowercase maps to 'A'

        assert_ne!(panel.buffer()[1], COLOR_BG);

        let mut panel = DebugWindow::initialise(DEBUG_WIDTH, DEBUG_HEIGHT);
        panel.draw_char(0, 0, '~', COLOR_TEXT); // not in table

        assert!(panel.buffer().iter().all(|&p| p == COLOR_BG));
    }

    fn decode_row(buffer: &[u32], row: usize, max_chars: usize) -> String {
        let mut text = String::new();
        for c in 0..max_chars {
            let mut glyph = [0u8; 7];
            for (ry, row_bits) in glyph.iter_mut().enumerate() {
                for rx in 0..5 {
                    let px = 2 + c * PITCH_X + rx;
                    let py = row * PITCH_Y + ry;
                    let pixel = buffer[py * DEBUG_WIDTH + px];
                    if pixel == COLOR_TEXT || pixel == COLOR_ACCENT || pixel == COLOR_DIM {
                        *row_bits |= 0x10 >> rx;
                    }
                }
            }
            match FONT_5X7.iter().position(|g| *g == glyph) {
                Some(i) => text.push((0x20 + i) as u8 as char),
                None => text.push('?'),
            }
        }
        text
    }

    #[test]
    fn rendered_text_is_decodable() {
        let mut panel = DebugWindow::initialise(DEBUG_WIDTH, DEBUG_HEIGHT);
        let mut cpu = cpu();
        cpu.memory.insert_instruction(0x200, 0xD111);
        for (i, v) in [1, 2, 3, 4, 5, 6, 7, 8].iter().enumerate() {
            cpu.v[i] = *v;
        }
        cpu.vi = 0x234;
        cpu.sp = 2;
        cpu.stack[1] = 0x300;
        cpu.stack[2] = 0x400;
        cpu.delay_timer = 0x12;
        cpu.sound_timer = 0x34;
        panel.render(&cpu, true);

        let title = decode_row(panel.buffer(), 0, 15);
        assert_eq!(title, "CHIP-8 DEBUGGER");

        let status_line = decode_row(panel.buffer(), 1, 32);
        assert!(status_line.contains("PC=200"), "got: {}", status_line);
        assert!(status_line.contains("I=234"), "got: {}", status_line);
        assert!(status_line.contains("DT=12"), "got: {}", status_line);
        assert!(status_line.contains("ST=34"), "got: {}", status_line);

        let reg_line = decode_row(panel.buffer(), 3, 26);
        assert!(reg_line.contains("V1=02"), "got: {}", reg_line);
        assert!(reg_line.contains("V3=04"), "got: {}", reg_line);

        let disasm_line = decode_row(panel.buffer(), 11, 24);
        assert!(disasm_line.contains("DRW V1, V1, 1"), "got: {}", disasm_line);

        let mem_header = decode_row(panel.buffer(), 16, 14);
        assert!(mem_header.contains("MEM@I"), "got: {}", mem_header);

        // Memory data equals the opcodes that were written at 0x200.
        let mem_line = decode_row(panel.buffer(), 20, 29);
        assert!(mem_line.contains("D1 11"), "got: {}", mem_line);
    }
}