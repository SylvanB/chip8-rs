use rand::Rng;

use crate::{
    display::{DebugDisplay, Display},
    memory::Memory,
    opcode::OpCode,
};

#[derive(Debug)]
pub(crate) struct CPU {
    pub memory: Memory,
    pub display: Display,

    // General purpose addresses
    pub v: [u8; 15],

    // Used for program flags
    pub vf: u8,

    // Commonly used to store memory addresses
    pub vi: u16,

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
            vf: 0x0,
            vi: 0x0,
            delay: 0x0,
            sound_timer: 0x0,
            pc: 0x200,
            sp: 0x0,
            stack: [0x0; 16],
        }
    }

    pub fn execute_next_instruction(&mut self) {
        if let Some(op) = self.get_op() {
            self.execute_op(op);
        }
    }

    pub fn execute(&mut self) {
        while let Some(op) = self.get_op() {
            self.execute_op(op);
        }
    }

    fn execute_op(&mut self, op: OpCode) {
        match op.raw() {
            0x0000..=0x0ff => match op.raw() {
                0x00e0 => self.cls(),
                0x00ee => self.ret(),
                _ => {} // no-op
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
            0xd000..=0xdfff => self.drw(&op),
            0xe000..=0xefff => todo!(),
            0xf000..=0xffff => self.ops_f(&op),
            _ => {} // no-op
        };
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
            self.shl(&op);
        } else {
        }
    }

    fn ops_f(&mut self, op: &OpCode) {
        if op.raw() & 0x00FF == 0x1E {
            self.add_i(&op);
        } else if op.raw() & 0x00FF == 0x29 {
            todo!();
        } else if op.raw() & 0x00FF == 0x33 {
            self.ld_b(&op);
        } else if op.raw() & 0x00FF == 0x55 {
            self.ld_mem_i(&op);
        }
    }

    /// The values of I and Vx are added, and the results are stored in I.
    fn add_i(&mut self, op: &OpCode) {
        self.vi += self.v[op.x() as usize] as u16;
    }

    /// Stores the BCD representation of the number in Vx in memory locations Vi, Vi + 1, Vi + 2
    fn ld_b(&mut self, op: &OpCode) {
        let value = self.v[op.x() as usize];
        let hund = (value / 100) % 10;
        let tens = (value / 10) % 10;
        let ones = value % 10;

        let addr = self.vi;
        self.memory.write(addr as usize, hund);
        self.memory.write((addr + 1) as usize, tens);
        self.memory.write((addr + 2) as usize, ones);
    }

    /// Stores the values of the register in the range 0..=Vx starting at the address pointed at by Vi.
    fn ld_mem_i(&mut self, op: &OpCode) {
        let max = op.x();

        for x in 0..=max {
            let val = self.v[x as usize];
            self.memory.write((self.vi + (x as u16)) as usize, val);
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
            self.vf = 1;
        } else {
            self.vf = 0;
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
            self.vf = 1;
        } else {
            self.vf = 0;
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
            self.vf = 1;
        } else {
            self.vf = 0;
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
            self.vf = 1;
        } else {
            self.vf = 0;
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
            self.vf = 1;
        } else {
            self.vf = 0;
        }
    }

    /// Skip if a register value is not equal to the value of another register
    ///
    /// Given a op of `0x3[X][Y]0` if the  value of `V[X] != V[Y]` skip the next instruction
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

        self.vi = addr;
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

    fn drw(&mut self, op: &OpCode) {
        let x = op.x();
        let y = op.y();
        let n = (op.raw() & 0x000F) as u8;

        let mut sprite = vec![0; n as _];

        for x in 0..n {
            sprite[x as usize] = self.memory.get((self.vi + (x as u16)) as _);
        }

        self.vf = self.display.display_sprite((&x, &y), &sprite) as u8;
    }
}

impl DebugDisplay for CPU {
    fn view_state(&self) {
        println!("Registers:");
        println!("{:#x?}", self.v);
        println!("vf: {:#x?} vi: {:#x?}", self.vf, self.vi);
        println!("pc: {:#x?} sp: {:#x?}", self.pc, self.sp);
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        display::{DebugDisplay, Display},
        memory::Memory,
    };

    use super::CPU;

    fn get_cpu() -> CPU {
        CPU::initialise(Memory::initialise(), Display::initialise())
    }

    fn load_new_cpu_with_instruction(op: u16) -> CPU {
        let mut cpu = get_cpu();
        cpu.memory.data[0x200] = ((op & 0xFF00) >> 8) as u8;
        cpu.memory.data[0x201] = (op & 0x00FF) as u8;

        cpu
    }

    #[test]
    fn ret() {
        let mut cpu = load_new_cpu_with_instruction(0x00EE);

        cpu.sp = 1;
        cpu.stack[1] = 0x500;

        cpu.execute_next_instruction();

        assert_eq!(cpu.sp, 0);
        assert_eq!(cpu.pc, 0x500);
    }

    #[test]
    fn cls() {
        // Do this some other time when we actually have shit that displays
    }

    #[test]
    fn jp() {
        let mut cpu = load_new_cpu_with_instruction(0x1666);

        cpu.execute_next_instruction();

        assert_eq!(cpu.pc, 0x666);
    }

    #[test]
    fn call() {
        let mut cpu = load_new_cpu_with_instruction(0x2666);

        cpu.execute_next_instruction();

        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.stack[cpu.sp as usize], 0x202);
        assert_eq!(cpu.pc, 0x666);
    }

    #[test]
    fn se_jumps_when_equal() {
        let mut cpu = load_new_cpu_with_instruction(0x3000);

        cpu.execute_next_instruction();

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn se_doesnt_jump_when_not_equal() {
        let mut cpu = load_new_cpu_with_instruction(0x3066);

        cpu.execute_next_instruction();

        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn sne_jumps_when_not_equal() {
        let mut cpu = load_new_cpu_with_instruction(0x4066);

        cpu.execute_next_instruction();

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn sne_doesnt_jump_when_equal() {
        let mut cpu = load_new_cpu_with_instruction(0x4000);

        cpu.execute_next_instruction();

        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn se_r_jumps_when_equal() {
        let mut cpu = load_new_cpu_with_instruction(0x5000);

        cpu.execute_next_instruction();

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn se_r_doesnt_jump_when_not_equal() {
        let mut cpu = load_new_cpu_with_instruction(0x5010);
        cpu.v[1] = 1;

        cpu.execute_next_instruction();

        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn ld_r() {
        let mut cpu = load_new_cpu_with_instruction(0x6066);

        cpu.execute_next_instruction();

        assert_eq!(cpu.v[0], 0x66);
    }

    #[test]
    fn add() {
        let mut cpu = load_new_cpu_with_instruction(0x7066);

        cpu.v[0] = 0x10;
        cpu.execute_next_instruction();

        assert_eq!(cpu.v[0], 0x76);
    }

    #[test]
    fn ld_xy() {
        let mut cpu = load_new_cpu_with_instruction(0x8010);

        cpu.v[1] = 0x10;
        cpu.execute_next_instruction();

        assert_eq!(cpu.v[0], 0x10);
    }

    #[test]
    fn or_xy() {
        let mut cpu = load_new_cpu_with_instruction(0x8011);
        cpu.v[0] = 0x01;
        cpu.v[1] = 0x10;

        cpu.execute_next_instruction();

        assert_eq!(cpu.v[0], 0x11);
    }

    #[test]
    fn or_xy_inverse() {
        let mut cpu = load_new_cpu_with_instruction(0x8011);
        cpu.v[0] = 0x11;
        cpu.v[1] = 0x11;

        cpu.execute_next_instruction();

        assert_eq!(cpu.v[0], 0x11);
    }

    #[test]
    fn and_xy() {
        let mut cpu = load_new_cpu_with_instruction(0x8012);
        cpu.v[0] = 0x01;
        cpu.v[1] = 0x10;

        cpu.execute_next_instruction();

        assert_eq!(cpu.v[0], 0x00);
    }

    #[test]
    fn xor_xy() {
        let mut cpu = load_new_cpu_with_instruction(0x8013);
        cpu.v[0] = 0x11;
        cpu.v[1] = 0x11;

        cpu.execute_next_instruction();

        assert_eq!(cpu.v[0], 0x00);
    }

    #[test]
    fn xor_xy_inverse() {
        let mut cpu = load_new_cpu_with_instruction(0x8013);
        cpu.v[0] = 0x01;
        cpu.v[1] = 0x10;

        cpu.execute_next_instruction();

        assert_eq!(cpu.v[0], 0x11);
    }

    #[test]
    fn add_xy_no_carry() {
        let mut cpu = load_new_cpu_with_instruction(0x8014);
        cpu.v[0] = 0xF0;
        cpu.v[1] = 0x0F;

        cpu.execute_next_instruction();

        assert_eq!(cpu.v[0], 0xFF);
        assert_eq!(cpu.vf, 0x0);
    }

    #[test]
    fn add_xy_carry() {
        let mut cpu = load_new_cpu_with_instruction(0x8014);
        cpu.v[0] = 0xFF;
        cpu.v[1] = 0x01;

        cpu.execute_next_instruction();

        assert_eq!(cpu.v[0], 0x0);
        assert_eq!(cpu.vf, 0x1);
    }

    // TODO: Define what should occur when overflow subtraction occurs in the CPU?
    // Doesnt seem to be defined in the spec very clearly.

    // #[test]
    // fn sub_xy_no_borrow() {
    //     let mut cpu = load_new_cpu_with_instruction(0x8015);
    //     cpu.v[0] = 0xF0;
    //     cpu.v[1] = 0x10;

    //     cpu.execute_next_instruction();

    //     assert_eq!(cpu.v[0], 0xE0);
    //     assert_eq!(cpu.vf, 0x1);
    // }

    // #[test]
    // fn sub_xy_borrow() {
    //     let mut cpu = load_new_cpu_with_instruction(0x8015);
    //     cpu.v[0] = 0x10;
    //     cpu.v[1] = 0x20;

    //     cpu.execute_next_instruction();

    //     assert_eq!(cpu.v[0], 0x0);
    //     assert_eq!(cpu.vf, 0x0);
    // }

    // TODO: Same as previous sub_xy tests
    // Need to define behavior more fully.
    // #[test]
    // fn subn() {}

    #[test]
    fn shr() {
        let mut cpu = load_new_cpu_with_instruction(0x8006);
        cpu.v[0] = 0x08;

        cpu.execute_next_instruction();

        assert_eq!(cpu.v[0], 0x4);
    }

    #[test]
    fn shr_vf_set_when_odd() {
        let mut cpu = load_new_cpu_with_instruction(0x8006);
        cpu.v[0] = 0x09;

        cpu.execute_next_instruction();

        assert_eq!(cpu.v[0], 0x4);
        assert_eq!(cpu.vf, 0x1);
    }

    // TODO: Define SHL behavior when Vx > 128
    // Again, more behavior that im unsure what is actually expected
    // SHL when used on a numbe > 128 will result in a 8-bit number overflow
    // What should happen here? shitcan the bits? use set it to 255?
    // #[test]
    // fn shl() {
    //     let mut cpu = load_new_cpu_with_instruction(0x800E);
    //     cpu.v[0] = 0x08;

    //     cpu.execute_next_instruction();

    //     assert_eq!(cpu.v[0], 0x10);
    // }

    // #[test]
    // fn shl_vf_set_when_over_128() {
    //     let mut cpu = load_new_cpu_with_instruction(0x800E);
    //     cpu.v[0] = 0b10001111;

    //     cpu.execute_next_instruction();

    //     assert_eq!(cpu.v[0], 0x10);
    // }

    #[test]
    fn sne_xy_skip_when_not_equal() {
        let mut cpu = load_new_cpu_with_instruction(0x9010);
        cpu.v[0] = 0x08;
        cpu.v[1] = 0x80;

        cpu.execute_next_instruction();

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn sne_xy_dont_skip_when_equal() {
        let mut cpu = load_new_cpu_with_instruction(0x9010);
        cpu.v[0] = 0x08;
        cpu.v[1] = 0x08;

        cpu.execute_next_instruction();

        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn ld_i() {
        let mut cpu = load_new_cpu_with_instruction(0xA666);

        cpu.execute_next_instruction();

        assert_eq!(cpu.vi, 0x666);
    }

    #[test]
    fn jp_v0() {
        let mut cpu = load_new_cpu_with_instruction(0xB666);
        cpu.v[0] = 0x4;

        cpu.execute_next_instruction();

        assert_eq!(cpu.pc, 0x66A);
    }

    // TODO: PEPELAUGH somehow deal with the rng.
    // #[test]
    // fn rnd() {
    //     let mut cpu = load_new_cpu_with_instruction(0xC066);
    //     cpu.v[0] = 0x4;

    //     cpu.execute_next_instruction();

    //     assert_eq!(cpu.pc, 0x66A);
    // }

    #[test]
    fn drw() {
        let mut cpu = get_cpu();

        cpu.memory.data[0x600] = 0xFF;
        cpu.vi = 0x600;
        cpu.memory.insert_instruction(0x200, 0xD111);

        cpu.execute_next_instruction();
        cpu.display.view_state();

        assert_eq!(cpu.display.screen[1][1], true);
        assert_eq!(cpu.display.screen[1][2], true);
        assert_eq!(cpu.display.screen[1][3], true);
        assert_eq!(cpu.display.screen[1][4], true);
        assert_eq!(cpu.display.screen[1][5], true);
        assert_eq!(cpu.display.screen[1][6], true);
        assert_eq!(cpu.display.screen[1][7], true);
        assert_eq!(cpu.display.screen[1][8], true);
    }

    #[test]
    fn add_i() {
        let mut cpu = load_new_cpu_with_instruction(0xF01E);
        cpu.vi = 0x6;
        cpu.v[0] = 0x4;

        cpu.execute_next_instruction();

        assert_eq!(cpu.vi, 0xA);
    }

    #[test]
    fn ld_b() {
        let mut cpu = load_new_cpu_with_instruction(0xF033);
        cpu.vi = 0x600;
        cpu.v[0] = 123;

        cpu.execute_next_instruction();

        assert_eq!(cpu.memory.get(0x600), 1);
        assert_eq!(cpu.memory.get(0x601), 2);
        assert_eq!(cpu.memory.get(0x602), 3);
    }

    #[test]
    fn ld_mem_i() {
        let mut cpu = load_new_cpu_with_instruction(0xF455);
        cpu.vi = 0x600;
        cpu.v[0] = 0x1;
        cpu.v[1] = 0x2;
        cpu.v[2] = 0x3;
        cpu.v[3] = 0x4;
        cpu.v[4] = 0x5;

        cpu.execute_next_instruction();

        assert_eq!(cpu.memory.get(0x600), 0x1);
        assert_eq!(cpu.memory.get(0x601), 0x2);
        assert_eq!(cpu.memory.get(0x602), 0x3);
        assert_eq!(cpu.memory.get(0x603), 0x4);
        assert_eq!(cpu.memory.get(0x604), 0x5);
    }
}
