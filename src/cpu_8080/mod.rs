static CYCLES: [u8; 256] = [4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, //0x00..0x0f
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, //0x10..0x1f
    4, 10, 16, 5, 5, 5, 7, 4, 4, 10, 16, 5, 5, 5, 7, 4, //etc
    4, 10, 13, 5, 10, 10, 10, 4, 4, 10, 13, 5, 5, 5, 7, 4,

    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, //0x40..0x4f
    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
    7, 7, 7, 7, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 7, 5,

    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, //0x80..8x4f
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,

    11, 10, 10, 10, 17, 11, 7, 11, 11, 10, 10, 10, 10, 17, 7, 11, //0xc0..0xcf
    11, 10, 10, 10, 17, 11, 7, 11, 11, 10, 10, 10, 10, 17, 7, 11,
    11, 10, 10, 18, 17, 11, 7, 11, 11, 5, 10, 5, 17, 17, 7, 11,
    11, 10, 10, 4, 17, 11, 7, 11, 11, 5, 10, 4, 17, 17, 7, 11];

static ADVANCE: [u8; 256] = [1, 3, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1,
    1, 3, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1,
    1, 3, 3, 1, 1, 1, 2, 1, 1, 1, 3, 1, 1, 1, 2, 1,
    1, 3, 3, 1, 1, 1, 2, 1, 1, 1, 3, 1, 1, 1, 2, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 3, 3, 3, 1, 2, 1, 1, 1, 3, 3, 3, 3, 2, 1,
    1, 1, 3, 2, 3, 1, 2, 1, 1, 1, 3, 2, 3, 3, 2, 1,
    1, 1, 3, 1, 3, 1, 2, 1, 1, 1, 3, 1, 3, 3, 2, 1,
    1, 1, 3, 1, 3, 1, 2, 1, 1, 1, 3, 1, 3, 3, 2, 1]; //TODO adjust movement for jumps


struct Flags {
    z: bool, //zero
    s: bool, //sign
    p: bool, //parity
    cy: bool, //carry
    ac: bool, //aux carry not used in space invaders
}

impl Flags {
    fn new() -> Flags {
        Flags {
            z: false,
            s: false,
            p: false,
            cy: false,
            ac: false,
        }
    }
}

pub struct State8080 {
    reg: Vec<u8>, //a, b, c, d, e, h, l
    sp: usize, //stack pointer traditionally u16
    pc: usize, //Traditionally u16, but we're trying to avoid many casts
    memory: Vec<u8>,
    flags: Flags,
    int_enable: u8,
}

impl State8080 {
    pub fn new(vec: Vec<u8>) -> State8080 {
        State8080 {
            reg: vec![0,0,0,0,0,0,0],
            sp: 0,
            pc: 0, //program counter
            memory: vec,
            flags: Flags::new(),
            int_enable: 0,
        }
    }
    /*
    Utility functions
    */

    ///Returns true if the number of 1s in the given number is even, else false
    fn parity(number: u8) -> bool {
        number.count_ones() % 2 == 0
    }

    ///Moves the memory pointer forward by the given number of positions
    pub fn advance(&mut self, value: u8) {
        self.pc += value as usize;
    }

    fn get_hl(&self) -> u8 {
        let significant_bits = (self.reg[5] as u16).wrapping_shl(4);
        self.memory[(significant_bits + self.reg[6] as u16) as usize]
    }

    fn set_hl(&mut self, value: u8) {
        let significant_bits = (self.reg[5] as u16).wrapping_shl(4);
        self.memory[(significant_bits + self.reg[6] as u16) as usize] = value;
    }

    fn get_hl_index(&self) -> usize {
        let significant_bits = (self.reg[5] as u16).wrapping_shl(4);
        (significant_bits + self.reg[6] as u16) as usize
    }

    /*
    8 bit arithmetic/logical instructions
    Functions ending with reg use values from the registers. Functions ending in hl use registers
    h and l as a 16-bit pointer to an 8-bit value in memory.
    */

    fn add(&mut self, number: u8, use_carry: bool) {
        let mut answer = self.reg[0] as u16 + number as u16;
        if use_carry {
            if self.flags.cy {
                answer += 0x01;
            }
        }
        self.flags.z = (answer & 0xff) == 0;
        self.flags.s = (answer & 0x80) != 0;
        self.flags.cy = answer > 0xff;
        self.flags.ac = ((self.reg[0] & 0x0f) + (number & 0x0f)) > 0x0f;
        self.flags.p = State8080::parity(answer as u8 & 0xff);
        self.reg[0] = (answer & 0xff) as u8;
    }

    ///Adds the value from the memory index referenced by H and L combined as a 16-bit pointer to
    /// the accumulator.
    pub fn add_hl(&mut self, use_carry: bool) {
        let number = self.get_hl();
        self.add(number, use_carry);
    }

    ///Adds the value from the given register index to the accumulator register.
    pub fn add_reg(&mut self, reg_num: usize, use_carry: bool) {
        let number = self.reg[reg_num];
        self.add(number, use_carry);
    }

    fn cmp(&mut self, number: u8) {
        let acc = self.reg[0]; //saving this value
        self.sub(number, false); //I believe carry should be ignored, docs didn't specify.
        self.reg[0] = acc; //reverting this
        if (number & 0x80) ^ (acc & 0x80) == 1 { //If they differ in sign
            self.flags.cy = !self.flags.cy; //Reverse the carry flag
        }
        //Unknown if actions should be taken for aux carry
    }

    pub fn cmp_hl(&mut self) {
        let number = self.get_hl();
        self.cmp(number);
    }

    pub fn cmp_reg(&mut self, reg_num: usize) {
        let number = self.reg[reg_num];
        self.cmp(number);
    }

    fn log_and(&mut self, number: u8) {
        let answer = number & self.reg[0]; //and with acc
        self.set_logic_flags(answer);
        self.reg[0] = answer;
    }

    pub fn log_and_hl(&mut self) {
        let number = self.get_hl();
        self.log_and(number);
    }

    pub fn log_and_reg(&mut self, reg_num: usize) {
        let number = self.reg[reg_num];
        self.log_and(number);
    }

    fn log_xor(&mut self, number: u8) {
        let answer = number ^ self.reg[0];
        self.set_logic_flags(answer);
        self.reg[0] = answer;
    }

    pub fn log_xor_hl(&mut self) {
        let number = self.get_hl();
        self.log_xor(number);
    }

    pub fn log_xor_reg(&mut self, reg_num: usize) {
        let number = self.reg[reg_num];
        self.log_xor(number);
    }

    fn log_or(&mut self, number: u8) {
        let answer = number | self.reg[0];
        self.set_logic_flags(answer);
        self.reg[0] = answer;
    }

    pub fn log_or_hl(&mut self) {
        let number = self.get_hl();
        self.log_or(number);
    }

    pub fn log_or_reg(&mut self, reg_num: usize) {
        let number = self.reg[reg_num];
        self.log_or(number);
    }

    fn sub(&mut self, number: u8, use_carry: bool) {
        let mut answer = self.reg[0] - number;
        if use_carry {
            if self.flags.cy {
                answer -= 0x01;
            }
        }
        self.flags.z = (answer & 0xff) == 0;
        self.flags.s = (answer &0x80) != 0;
        self.flags.cy = number > self.reg[0]; //Not sure
        self.flags.ac = (number & 0x0f) > (self.reg[0] & 0x0f); //Not sure
        self.flags.p = State8080::parity(answer);
        self.reg[0] = answer;
    }

    pub fn sub_hl(&mut self, use_carry: bool) {
        let number = self.get_hl();
        self.sub(number, use_carry);
    }

    pub fn sub_reg(&mut self, reg_num: usize, use_carry: bool) {
        let number = self.reg[reg_num];
        self.sub(number, use_carry);
    }

    fn set_logic_flags(&mut self, answer: u8) {
        self.flags.z = (answer & 0xff) == 0;
        self.flags.s = (answer & 0x80) != 0;
        self.flags.cy = false; //Carry always reset
        self.flags.p = State8080::parity(answer);
    }

    ///Increments the given register, or HL if 7.
    pub fn increment(&mut self, reg_num: usize) {
        if reg_num == 7 {
            let hl = self.get_hl();
            self.set_hl(hl + 1);
            let answer = self.get_hl();
            self.inc_dec_flags(answer);
        } else {
            self.reg[reg_num] += 1;
            let answer = self.reg[reg_num];
            self.inc_dec_flags(answer);
        }
    }

    ///Decrements the given register, or HL if 7
    pub fn decrement(&mut self, reg_num: usize) {
        if reg_num == 7 {
            let hl = self.get_hl();
            self.set_hl(hl - 1);
            let answer = self.get_hl();
            self.inc_dec_flags(answer);
        } else {
            self.reg[reg_num] -= 1;
            let answer = self.reg[reg_num];
            self.inc_dec_flags(answer);
        }
    }

    ///Sets the flags for the increment and decrement operations
    fn inc_dec_flags(&mut self, answer: u8) {
        self.flags.s = (answer & (0x80)) == 0x80;
        self.flags.z = answer == 0;
        self.flags.p = State8080::parity(answer);
        self.flags.ac = (answer > 0x0F) && (answer - 1 < 0x0F); //This may be wrong
    }

    ///Bitwise not to the accumulator
    pub fn complement_acc(&mut self) {
        self.reg[0] = !self.reg[0];
    }
    ///Reverses the carry bit
    pub fn complement_carry(&mut self) {
        self.flags.cy = !self.flags.cy;
    }

    ///Sets the carry bit to 1
    pub fn set_carry(&mut self, b: bool) {
        self.flags.cy = b;
    }

    ///Rotates the acc one bit to the left. If use_carry is true, then the carry is used as an
    /// additional bit. If it is false, then carry is still affected, but the most significant bit
    /// wraps to the least significant location anyway.
    pub fn rotate_acc_left(&mut self, use_carry: bool) {
        let mut acc = self.reg[0];
        if use_carry {
            let carry = self.flags.cy;
            self.flags.cy = (acc & 0x80) == 0x80;
            acc = acc.wrapping_shl(1);
            if carry {
                acc = acc | 0x01;
            }
        } else {
            self.flags.cy = (acc & 0x80) == 0x80; //most sig bit is moved into carry
            acc = acc.wrapping_shl(1);
            if self.flags.cy {
                acc = acc | 0x01; //The sig bit is still wrapped around
            }
        }
        self.reg[0] = acc;
    }
    ///Rotates the acc one bit to the right. If use_carry is true, then the carry is used as an
    /// additional bit. If it is false, then carry is still affected, but the least significant bit
    /// wraps to the most significant location anyway.
    pub fn rotate_acc_right(&mut self, use_carry: bool) {
        let mut acc = self.reg[0];
        if use_carry {
            let carry = self.flags.cy;
            self.flags.cy = (acc & 0x01) == 0x01;
            acc = acc.wrapping_shr(1);
            if carry {
                acc = acc | 0x80;
            }
        } else {
            self.flags.cy = (acc & 0x01) == 0x01;
            acc = acc.wrapping_shr(1);
            if self.flags.cy {
                acc = acc | 0x08; //The least sig bit is still wrapped around
            }
        }
        self.reg[0] = acc;
    }
    ///Transforms acc into binary-coded decimal form using a specific algorithm.
    pub fn decimal_adjust_acc(&mut self) {
        let mut acc = self.reg[0];
        let mut least_sig = acc & 0x0F;
        if least_sig > 9 || self.flags.ac {
            acc += 6;
        }
        if least_sig > 0x0F {
            self.flags.ac = true
        } else {
            self.flags.ac = false;
        }
        least_sig = least_sig & 0x0F;
        let mut most_sig = acc & 0xF0;
        most_sig.wrapping_shr(4);
        if most_sig > 9 || self.flags.cy {
            most_sig += 6;
        }
        if most_sig > 0x0F {
            self.flags.cy = true;
        }
        most_sig = most_sig.wrapping_shl(4);
        self.reg[0] = most_sig | least_sig;
    }

    /*
    jumps and calls
    */
    ///Moves pc to the given location, directing the program to that byte of memory
    pub fn jump(&mut self, least_sig: u8, most_sig: u8) {
        let location: usize = (((most_sig as u16).wrapping_shl(8)) | least_sig as u16) as usize;
        self.pc = location;
    }
    ///Jumps if the given conditional function returns true
    pub fn jump_if(&mut self, least_sig: u8, most_sig: u8, f: fn(&State8080) -> bool) {
        if f(self) {
            self.jump(least_sig, most_sig);
        }
    }

    /*
    load, store, mov codes
    */
    ///Takes the source reg or hl and moves it to the destination reg or hl, overriding the old
    /// value. The index value of 7 is considered to be HL. The source value is unchanged.
    pub fn mov(&mut self, source: usize, dest: usize) {
        let source_value: u8;
        if source == 7 {
            source_value = self.get_hl();
        } else {
            source_value = self.reg[source];
        }
        if dest == 7 {
            self.set_hl(source_value);
        } else {
            self.reg[dest] = source_value;
        }
    }
    ///Pushes the given bytes onto the stack
    pub fn stack_push(&mut self, least_sig: u8, most_sig: u8) {
        self.memory[self.sp - 1] = most_sig;
        self.memory[self.sp - 2] = least_sig;
        self.sp -= 2;
    }
    ///Pops the given bytes from the stack, placing the least significant in L and the most
    /// significant in H.
    pub fn stack_pop(&mut self) {
        self.reg[6] = self.memory[self.sp];
        self.reg[5] = self.memory[self.sp + 1];
        self.sp += 2;
    }
    ///Pushes a 16-bit register pair onto the stack. The number supplied is the first in the pair
    pub fn push_reg(&mut self, first: usize) {
        let value1 = self.reg[first + 1];
        let value2 = self.reg[first];
        self.stack_push(value1, value2); //first is most significant
    }
    ///Calls a subroutine by jumping to the given location in memory, but pushes the memory location
    /// after the call onto the stack, so it can be popped off when the subroutine has ended
    pub fn call(&mut self, least_sig: u8, most_sig: u8) {
        let address = (self.pc + 3) as u16;
        self.stack_push((address & 0xFF as u16) as u8, (address & 0xFF00 as u16) as u8);
        self.jump(least_sig, most_sig);
    }
    ///Calls a subroutine if the condition function returns true
    pub fn call_if(&mut self, least_sig: u8, most_sig: u8, f: fn(&State8080) -> bool) {
        if f(self) {
            self.call(least_sig, most_sig);
        }
    }
    ///Returns from a subroutine call
    pub fn ret(&mut self) {
        self.stack_pop(); //Pop the memory location off the stack
        self.pc = self.get_hl_index(); //and set pc to that location
    }
    ///Returns from a subroutine call if the condition function returns true
    pub fn ret_if(&mut self, f: fn(&State8080) -> bool) {
        if f(self) {
            self.ret();
        }
    }

}

pub fn emulate(state: &mut State8080) {
    let func = |s: &State8080| -> bool {!s.flags.z };
    let opcode = state.memory[state.pc]; //Should never be out of bounds
    let second = *(state.memory.get(state.pc + 1).unwrap_or(&0));
    let third = *(state.memory.get(state.pc + 2).unwrap_or(&0));
    match opcode {
        0x00 => (),

        0x04 => state.increment(1),
        0x05 => state.decrement(1),
        0x07 => state.rotate_acc_left(false),
        0x0C => state.increment(2),
        0x0D => state.decrement(2),
        0x0F => state.rotate_acc_right(false),
        0x14 => state.increment(3),
        0x15 => state.decrement(3),
        0x17 => state.rotate_acc_left(true),
        0x1C => state.increment(4),
        0x1D => state.decrement(4),
        0x1F => state.rotate_acc_right(true),
        0x24 => state.increment(5),
        0x25 => state.decrement(5),
        0x27 => state.decimal_adjust_acc(), //DAA
        0x2C => state.increment(6),
        0x2D => state.decrement(6),
        0x2F => state.complement_acc(),
        0x34 => state.increment(7),
        0x35 => state.decrement(7),
        0x37 => state.set_carry(true),
        0x3C => state.increment(0),
        0x3D => state.decrement(0),
        0x3F => state.complement_carry(),

        //mov codes
        0x40 => state.mov(1, 1),
        0x41 => state.mov(1, 2),
        0x42 => state.mov(1, 3),
        0x43 => state.mov(1, 4),
        0x44 => state.mov(1, 5),
        0x45 => state.mov(1, 6),
        0x46 => state.mov(1, 7), //mov to HL
        0x47 => state.mov(1, 0),
        0x48 => state.mov(2, 1),
        0x49 => state.mov(2, 2),
        0x4A => state.mov(2, 3),
        0x4B => state.mov(2, 4),
        0x4C => state.mov(2, 5),
        0x4D => state.mov(2, 6),
        0x4E => state.mov(2, 7), //mov to HL
        0x4F => state.mov(2, 0),
        0x50 => state.mov(3, 1),
        0x51 => state.mov(3, 2),
        0x52 => state.mov(3, 3),
        0x53 => state.mov(3, 4),
        0x54 => state.mov(3, 5),
        0x55 => state.mov(3, 6),
        0x56 => state.mov(3, 7), //mov to HL
        0x57 => state.mov(3, 0),
        0x58 => state.mov(4, 1),
        0x59 => state.mov(4, 2),
        0x5A => state.mov(4, 3),
        0x5B => state.mov(4, 4),
        0x5C => state.mov(4, 5),
        0x5D => state.mov(4, 6),
        0x5E => state.mov(4, 7), //mov to HL
        0x5F => state.mov(4, 0),
        0x60 => state.mov(5, 1),
        0x61 => state.mov(5, 2),
        0x62 => state.mov(5, 3),
        0x63 => state.mov(5, 4),
        0x64 => state.mov(5, 5),
        0x65 => state.mov(5, 6),
        0x66 => state.mov(5, 7), //mov to HL
        0x67 => state.mov(5, 0),
        0x68 => state.mov(6, 1),
        0x69 => state.mov(6, 2),
        0x6A => state.mov(6, 3),
        0x6B => state.mov(6, 4),
        0x6C => state.mov(6, 5),
        0x6D => state.mov(6, 6),
        0x6E => state.mov(6, 7), //mov to HL
        0x6F => state.mov(6, 0),
        0x70 => state.mov(7, 1),
        0x71 => state.mov(7, 2),
        0x72 => state.mov(7, 3),
        0x73 => state.mov(7, 4),
        0x74 => state.mov(7, 5),
        0x75 => state.mov(7, 6),
        0x76 => (), //TODO HLT
        0x77 => state.mov(7, 0),
        0x78 => state.mov(0, 1),
        0x79 => state.mov(0, 2),
        0x7A => state.mov(0, 3),
        0x7B => state.mov(0, 4),
        0x7C => state.mov(0, 5),
        0x7D => state.mov(0, 6),
        0x7E => state.mov(0, 7), //mov to HL
        0x7F => state.mov(0, 0),
        //8 bit arithmetic / logic
        0x80 => state.add_reg(1, false), //Add B
        0x81 => state.add_reg(2, false),
        0x82 => state.add_reg(3, false),
        0x83 => state.add_reg(4, false), //Add E
        0x84 => state.add_reg(5, false),
        0x85 => state.add_reg(6, false), //Add L
        0x86 => state.add_hl(false), //Add (HL)
        0x87 => state.add_reg(0, false), //Add A
        0x88 => state.add_reg(1, true), //ADC B
        0x89 => state.add_reg(2, true),
        0x8A => state.add_reg(3, true),
        0x8B => state.add_reg(4, true),
        0x8C => state.add_reg(5, true),
        0x8D => state.add_reg(6, true),
        0x8E => state.add_hl(true),
        0x8F => state.add_reg(0, true),
        0x90 => state.sub_reg(1, false), //sub B
        0x91 => state.sub_reg(2, false),
        0x92 => state.sub_reg(3, false),
        0x93 => state.sub_reg(4, false), //sub E
        0x94 => state.sub_reg(5, false),
        0x95 => state.sub_reg(6, false), //sub L
        0x96 => state.sub_hl(false), //sub (HL)
        0x97 => state.sub_reg(0, false), //sub A
        0x98 => state.sub_reg(1, true), //SBB B
        0x99 => state.sub_reg(2, true),
        0x9A => state.sub_reg(3, true),
        0x9B => state.sub_reg(4, true),
        0x9C => state.sub_reg(5, true),
        0x9D => state.sub_reg(6, true),
        0x9E => state.sub_hl(true),
        0x9F => state.sub_reg(0, true),
        0xA0 => state.log_and_reg(1),
        0xA1 => state.log_and_reg(2),
        0xA2 => state.log_and_reg(3),
        0xA3 => state.log_and_reg(4),
        0xA4 => state.log_and_reg(5),
        0xA5 => state.log_and_reg(6),
        0xA6 => state.log_and_hl(),
        0xA7 => state.log_and_reg(0),
        0xA8 => state.log_xor_reg(1),
        0xA9 => state.log_xor_reg(2),
        0xAA => state.log_xor_reg(3),
        0xAB => state.log_xor_reg(4),
        0xAC => state.log_xor_reg(5),
        0xAD => state.log_xor_reg(6),
        0xAE => state.log_xor_hl(),
        0xAF => state.log_xor_reg(0),
        0xB0 => state.log_or_reg(1),
        0xB1 => state.log_or_reg(2),
        0xB2 => state.log_or_reg(3),
        0xB3 => state.log_or_reg(4),
        0xB4 => state.log_or_reg(5),
        0xB5 => state.log_or_reg(6),
        0xB6 => state.log_or_hl(),
        0xB7 => state.log_or_reg(0),
        0xB8 => state.cmp_reg(1),
        0xB9 => state.cmp_reg(2),
        0xBA => state.cmp_reg(3),
        0xBB => state.cmp_reg(4),
        0xBC => state.cmp_reg(5),
        0xBD => state.cmp_reg(6),
        0xBE => state.cmp_hl(),
        0xBF => state.cmp_reg(0),
        0xC2 => state.jump_if(second, third, if_not_zero),
        0xC3 => state.jump(second, third),
        0xC4 => state.call_if(second, third, if_not_zero),
        0xC5 => state.push_reg(1),
        0xC6 => state.add(second, false),

        0xC9 => state.ret(),
        0xCA => state.jump_if(second, third, if_zero),
        0xCB => state.jump(second, third), //Repeat of 0xC3, usage of this is not advised
        0xCC => state.call_if(second, third, if_zero),
        0xCD => state.call(second, third),
        0xCE => state.add(second, true),

        0xD6 => state.sub(second, false),
        0xDE => state.sub(second, true),
        0xE6 => state.log_and(second),
        0xEE => state.log_xor(second),
        0xF6 => state.log_or(second),
        0xFE => state.cmp(second),
        _ => (),
    };
    let cycles = CYCLES[opcode as usize];
    state.advance(ADVANCE[opcode as usize]);
    ()
}

fn if_zero(state: &State8080) -> bool {
    state.flags.z
}

fn if_not_zero(state: &State8080) -> bool {
    !state.flags.z
}

fn if_carry(state: &State8080) -> bool {
    state.flags.cy
}

fn if_not_carry(state: &State8080) -> bool {
    !state.flags.cy
}

fn if_parity(state: &State8080) -> bool {
    state.flags.p
}

fn if_not_parity(state: &State8080) -> bool {
    !state.flags.p
}