use crate::components::{AddressableStorage, Register, Memory};
use crate::Instruction;
use rand::{self, Rng};

pub struct Chip {
    pub memory: Memory,
    pub register: Register,
    pub program_counter: u16,
    pub stack: Vec<u16>,
}

impl Chip {
    pub fn new() -> Chip {
        Chip {
            register: Register::new(),
            memory: Memory::new(),
            program_counter: 0,
            stack: Vec::new(),
        }
    }

    pub fn with_register_values(values: &[u8]) -> Chip {
        let mut chip = Chip::new();
        chip.register = Register::with_values(values);
        chip
    }

    pub fn execute(&mut self, instruction: &Instruction) {
        println!("{0:<15?} ", instruction);
        match *instruction {
            Instruction::Sys(_) => unimplemented!(),
            Instruction::Cls => unimplemented!(),
            Instruction::Ret => {
                if let Some(addr) = self.stack.pop() {
                    self.program_counter = addr & 0xFFF;
                }
            }
            Instruction::Jp(addr) => self.program_counter = addr,
            Instruction::Call(addr) => {
                self.stack.push(self.program_counter);
                self.program_counter = addr & 0xFFF;
            }
            Instruction::SeByte(vx, value) => {
                if self.register.get(vx) == value {
                    self.program_counter += 2;
                }
            }
            Instruction::SneByte(vx, value) => {
                if self.register.get(vx) != value {
                    self.program_counter += 2;
                }
            }
            Instruction::Se(vx, vy) => {
                if self.register.get(vx) == self.register.get(vy) {
                    self.program_counter += 2;
                }
            }
            Instruction::Sne(vx, vy) => {
                if self.register.get(vx) != self.register.get(vy) {
                    self.program_counter += 2;
                }
            }
            Instruction::LdByte(vx, value) => self.register.set(vx, value),
            Instruction::AddByte(vx, value) => {
                let value_x = self.register.get(vx) as u16;
                let sum: u8 = ((value_x + value as u16) & 255) as u8;
                self.register.set(vx, sum)
            }
            Instruction::Ld(vx, vy) => {
                let value = self.register.get(vy);
                self.register.set(vx, value);
            }
            Instruction::Or(vx, vy) => {
                let value_x = self.register.get(vx);
                let value_y = self.register.get(vy);
                self.register.set(vx, value_x | value_y);
                bitwise!(value_x, |, value_y);
            }
            Instruction::And(vx, vy) => {
                let value_x = self.register.get(vx);
                let value_y = self.register.get(vy);
                self.register.set(vx, value_x & value_y);
                bitwise!(value_x, &, value_y);
            }
            Instruction::Xor(vx, vy) => {
                let value_x = self.register.get(vx);
                let value_y = self.register.get(vy);
                self.register.set(vx, value_x ^ value_y);
                bitwise!(value_x, ^, value_y);
            }
            Instruction::Add(vx, vy) => {
                let value_x = self.register.get(vx) as u16;
                let value_y = self.register.get(vy) as u16;
                let sum: u16 = value_x + value_y;
                let overflow = if sum > 255 { 1 } else { 0 };
                self.register.set(0xF, overflow);
                self.register.set(vx, (sum & 255) as u8);
            }
            Instruction::Sub(vx, vy) => {
                let value_x = self.register.get(vx) as i16;
                let value_y = self.register.get(vy) as i16;
                let no_borrow = if value_x > value_y { 1 } else { 0 };
                self.register.set(0xF, no_borrow);
                self.register.set(vx, ((value_x - value_y) & 255) as u8);
            }
            Instruction::Shr(vx) => {
                let value_x = self.register.get(vx);
                let least_sig_bit = value_x & 0b1;
                self.register.set(0xF, least_sig_bit);
                self.register.set(vx, value_x >> 1);
            }
            Instruction::Subn(vx, vy) => {
                let value_x = self.register.get(vx) as i16;
                let value_y = self.register.get(vy) as i16;
                let no_borrow = if value_y > value_x { 1 } else { 0 };
                self.register.set(0xF, no_borrow);
                self.register.set(vx, ((value_y - value_x) & 255) as u8);
            }
            Instruction::Shl(vx) => {
                let value_x = self.register.get(vx);
                let most_sig_bit = (value_x & 0b10000000) >> 7;
                self.register.set(0xF, most_sig_bit);
                self.register.set(vx, value_x << 1);
            }
            Instruction::Ldi(value) => {
                self.register.i = value;
            }
            Instruction::JpV0(addr) => {
                self.program_counter = self.register.get(0) as u16 + addr;
            }
            Instruction::Rnd(vx, mask) => {
                let random: u8 = rand::thread_rng().gen::<u8>();
                self.register.set(vx, random & mask);
            }
            Instruction::LdVxDelay(vx) => {
                let delay = self.register.delay;
                self.register.set(vx, delay);
            }
            Instruction::LdDelayVx(vx) => {
                self.register.delay = self.register.get(vx);
            }
        }
        println!("{:?}\n", self.register.values);
    }
}
