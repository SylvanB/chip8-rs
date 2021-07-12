use rand::Rng;

use crate::{display::Display, memory::Memory, opcode::OpCode};

#[derive(Debug)]
pub(crate) struct CPU {
    pub memory: Memory,
    pub display: Display,

    // General purpose addresses
    pub v: [u8; 15],

    // Used for program flags
    pub VF: u8,

    // Commonly used to store memory addresses
    pub VI: u16,

    /*
       Delay register:
       "The delay timer is active whenever the delay timer register (DT) is non-zero.
        This timer does nothing more than subtract 1 from the value of DT at a rate of 60Hz.
        When DT reaches 0, it deactivates."
    */
    pub delay: u8,

    /*
        Sound timer register:
        "The sound timer is active whenever the sound timer register (ST) is non-zero.
         This timer also decrements at a rate of 60Hz, however, as long as ST's value is greater than zero, the Chip-8 buzzer will sound.
         When ST reaches zero, the sound timer deactivates."
    */
    pub sound_timer: u8,

    // Stores currently executing address
    pub pc: u16,

    // Point to the top of the stack
    pub sp: u8,

    // Stores the address that should be returned to once a subroutine has finished execution
    // This gives Chip-8 a max nested subroutine level of 16
    pub stack: [u16; 16],
}

impl CPU {
    pub fn initialise(memory: Memory, display: Display) -> Self {
        CPU {
            memory,
            display,
            v: [0; 15],
            VF: 0x0,
            VI: 0x0,
            delay: 0x0,
            sound_timer: 0x0,
            pc: 0x200,
            sp: 0x0,
            stack: [0x0; 16],
        }
    }

    pub fn execute(&mut self) {
        while let Some(op) = self.get_op() {
            match op.raw() {
                0x0000..=0x0ff => match op.raw() {
                    0x00e0 => self.cls(),
                    0x00ee => self.ret(),
                    _ => break, // _ => panic!("Unexpected opcode. {}", op),
                },
                0x1000..=0x1fff => self.jp(&op),
                0x2000..=0x2fff => self.call(&op),
                0x3000..=0x3fff => self.se(&op),
                0x4000..=0x4fff => self.sne(&op),
                0x5000..=0x5fff => self.se_r(&op),
                0x6000..=0x6fff => self.ld_r(&op),
                0x7000..=0x7fff => self.add(&op),
                0x8000..=0x8fff => self.ops_8(&op),
                0x9000..=0x9fff => self.sne_xy(&op),
                0xa000..=0xafff => self.ld_i(&op),
                0xb000..=0xbfff => self.jp_v0(&op),
                0xc000..=0xcfff => self.rnd(&op),
                0xd000..=0xdfff => todo!(),
                0xe000..=0xefff => todo!(),
                0xf000..=0xffff => todo!(),
                _ => break, // _ => panic!("Invalid op code."),
            };
            println!("{:#?}", &self);
        }
    }

    /// Get the next opcode
    ///
    /// Opcodes are constructed from 2 bytes, the most significant first (big endian)
    /// We fetch the next two values in memory and construct the opcode by shifting and bitwise AND'ing the bytes.
    fn get_op(&mut self) -> Option<OpCode> {
        let a = (self.memory.get(self.pc as _) as u16) << 8;
        self.pc += 1;
        let b = self.memory.get(self.pc as _) as u16;
        self.pc += 1;

        Some(OpCode::new((a | b) as u16))
    }

    /// Asks the Display to clear the screen
    fn cls(&mut self) {
        self.display.clear_screen();
    }

    /// Used to return from a subroutine
    fn ret(&mut self) {
        match self.stack.get(self.sp as usize) {
            Some(addr) => {
                self.pc = *addr;
                self.sp -= 1;
            }
            None => {}
        }
    }

    /// Jumps to a given memory location
    fn jp(&mut self, op: &OpCode) {
        self.pc = op.nnn();
    }

    /// Calls the subroutine at the specific address
    fn call(&mut self, op: &OpCode) {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = op.nnn();
    }

    /// Skip if a register value is equal to a given byte
    ///
    /// Given a op of `0x3[X][KK]` if the  value of `V[X] == [KK]` skip the next instruction
    fn se(&mut self, op: &OpCode) {
        if (self.v[op.x() as usize]) == op.kk() {
            self.pc += 2;
        }
    }

    /// Skip if a register value is not equal to a given byte
    ///
    /// Given a op of `0x3[X][KK]` if the  value of `V[X] != [KK]` skip the next instruction
    fn sne(&mut self, op: &OpCode) {
        if (self.v[op.x() as usize]) != op.kk() {
            self.pc += 2;
        }
    }

    /// Skip if the register `Vx` == `Vy`
    fn se_r(&mut self, op: &OpCode) {
        if self.v[op.x() as usize] == self.v[op.y() as usize] {
            self.pc += 2;
        }
    }

    /// Loads the value `kk` into the register `Vx`
    fn ld_r(&mut self, op: &OpCode) {
        self.v[op.x() as usize] = op.kk();
    }

    // Adds the value `kk` to the value in the register `Vx`
    fn add(&mut self, op: &OpCode) {
        self.v[op.x() as usize] += op.kk();
    }

    /// Loads the value in the register `Vy` into the register `Vx`
    fn ld_xy(&mut self, op: &OpCode) {
        self.v[op.x() as usize] = self.v[op.y() as usize];
    }

    fn ops_8(&mut self, op: &OpCode) {
        if op.raw() & 0x000F == 0x0 {
            self.ld_xy(&op);
        } else if op.raw() & 0x000F == 0x1 {
            self.or_xy(&op);
        } else if op.raw() & 0x000F == 0x2 {
            self.and_xy(&op);
        } else if op.raw() & 0x000F == 0x3 {
            self.xor_xy(&op);
        } else if op.raw() & 0x000F == 0x4 {
            self.add_xy(&op);
        } else if op.raw() & 0x000F == 0x5 {
            self.sub_xy(&op);
        } else if op.raw() & 0x000F == 0x6 {
            self.shr(&op);
        } else if op.raw() & 0x000F == 0x7 {
            self.subn_yx(&op);
        } else if op.raw() & 0x000F == 0xE {
            self.subn_yx(&op);
        } else {
        }
    }

    /// Performs a bitwise OR operation on the values in the registers `Vx` and `Vy` and stores the result in `Vx`
    fn or_xy(&mut self, op: &OpCode) {
        self.v[op.x() as usize] |= self.v[op.y() as usize];
    }

    /// Performs a bitwise AND operation on the values in the registers `Vx` and `Vy` and stores the result in `Vx`
    fn and_xy(&mut self, op: &OpCode) {
        self.v[op.x() as usize] &= self.v[op.y() as usize];
    }

    /// Performs a bitwise XOR operation on the values in the registers `Vx` and `Vy` and stores the result in `Vx`
    fn xor_xy(&mut self, op: &OpCode) {
        self.v[op.x() as usize] ^= self.v[op.y() as usize];
    }

    /// Adds the values of registers `Vx` and `Vy` and stores the lower byte into `Vx`
    ///
    /// If the result of the addition is greater than 255, the VF flag is set to 1 else 0;
    fn add_xy(&mut self, op: &OpCode) {
        let x = op.x();
        let y = op.y();

        let vx = self.v[x as usize] as u16;
        let vy = self.v[y as usize] as u16;

        let res = vx + vy;

        if res > 255 {
            self.VF = 1;
        } else {
            self.VF = 0;
        }

        self.v[x as usize] = (res & 0x00FF) as u8;
    }

    /// Subtracts the value of the register `Vy` from `Vx` and stores it in `Vx`
    ///
    /// If `Vx` > `Vy` set VF to 1, else 0
    fn sub_xy(&mut self, op: &OpCode) {
        let x = op.x();
        let y = op.y();

        let vx = self.v[x as usize] as u16;
        let vy = self.v[y as usize] as u16;

        let res = vx - vy;

        if vx > vy {
            self.VF = 1;
        } else {
            self.VF = 0;
        }

        self.v[x as usize] = (res & 0x00FF) as u8;
    }

    /// Subtracts the value of the register `Vy` from `Vx` and stores it in `Vx`
    ///
    /// If `Vx` > `Vy` set VF to 1, else 0
    fn subn_yx(&mut self, op: &OpCode) {
        let x = op.x();
        let y = op.y();

        let vx = self.v[x as usize] as u16;
        let vy = self.v[y as usize] as u16;

        let res = vy - vx;

        if vy > vx {
            self.VF = 1;
        } else {
            self.VF = 0;
        }

        self.v[x as usize] = (res & 0x00FF) as u8;
    }

    /// Shifts the value of `Vx` right by 1 bit
    ///
    /// If the least-significant bit of `Vx` is 1, then VF is set to 1, else 0.
    fn shr(&mut self, op: &OpCode) {
        // TODO: Allow the additional `Vy` register to be shifted optionally

        let x = op.x();
        let vx = self.v[x as usize];

        self.v[x as usize] = vx >> 1;

        if vx & 0x0001 == 1 {
            self.VF = 1;
        } else {
            self.VF = 0;
        }
    }

    /// Shifts the value of `Vx` left by 1 bit
    ///
    /// If the most-significant bit of `Vx` is 1, then VF is set to 1, else 0.
    fn shl(&mut self, op: &OpCode) {
        // TODO: Allow the additional `Vy` register to be shifted optionally

        let x = op.x();
        let vx = self.v[x as usize];

        self.v[x as usize] = vx << 1;

        if vx & 0x0001 == 1 {
            self.VF = 1;
        } else {
            self.VF = 0;
        }
    }

    /// Skip if a register value is not equal to a given byte
    ///
    /// Given a op of `0x3[X][KK]` if the  value of `V[X] != [KK]` skip the next instruction
    fn sne_xy(&mut self, op: &OpCode) {
        let x = op.x();
        let y = op.y();

        let vx = self.v[x as usize];
        let vy = self.v[y as usize];

        if vx != vy {
            self.pc += 2;
        }
    }

    fn ld_i(&mut self, op: &OpCode) {
        let addr = op.nnn();

        self.VI = addr;
    }

    fn jp_v0(&mut self, op: &OpCode) {
        let addr = op.nnn();

        self.pc = (self.v[0] as u16) + addr;
    }

    fn rnd(&mut self, op: &OpCode) {
        let x = op.x();
        let kk = op.kk();

        let rand_number: u8 = rand::thread_rng().gen_range(0..=255);
        let res = kk & rand_number;

        self.v[x as usize] = res;
    }

    fn drw(&mut self, op: &OpCode) {}
}
