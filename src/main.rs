#[derive(Debug)]
pub enum Instruction {
    /// 6xkk - LD Vx, byte
    /// Set Vx = kk.
    /// The interpreter puts the value kk into register Vx.
    LdByte(usize, u8),

    /// 8xy0 - LD Vx, Vy
    /// Set Vx = Vy.
    /// Stores the value of register Vy in register Vx.
    Ld(usize, usize), // Load

    /// 8xy1 - OR Vx, Vy
    /// Set Vx = Vx OR Vy.
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
    Or(usize, usize),

    /// 8xy2 - AND Vx, Vy
    /// Set Vx = Vx AND Vy.
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
    And(usize, usize),

    /// 8xy3 - XOR Vx, Vy
    /// Set Vx = Vx XOR Vy.
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
    Xor(usize, usize),

    /// 8xy4 - ADD Vx, Vy
    /// Set Vx = Vx + Vy, set VF = carry.
    /// The values of Vx and Vy are added together.
    /// If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
    /// Only the lowest 8 bits of the result are kept, and stored in Vx.
    Add(usize, usize),

    /// 8xy5 - SUB Vx, Vy
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    /// If Vx > Vy, then VF is set to 1, otherwise 0.
    /// Then Vy is subtracted from Vx, and the results stored in Vx.
    Sub(usize, usize),

    /// 8xy6 - SHR Vx {, Vy}
    /// Set Vx = Vx SHR 1.
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
    /// Then Vx is divided by 2.
    Shr(usize), // Shift Right

    /// 8xy7 - SUBN Vx, Vy
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    /// If Vy > Vx, then VF is set to 1, otherwise 0.
    /// Then Vx is subtracted from Vy, and the results stored in Vx.
    Subn(usize, usize),

    /// 8xyE - SHL Vx {, Vy}
    /// Set Vx = Vx SHL 1.
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
    /// Then Vx is multiplied by 2.
    Shl(usize), // Shift Left

    /// Unknown Instruction
    Unknown,
}

#[derive(Debug)]
pub struct Register {
    values: [u8; 16],
}

impl Register {
    pub fn set(&mut self, key: usize, value: u8) {
        self.values[key] = value;
    }

    pub fn get(&self, key: usize) -> u8 {
        self.values[key]
    }
}

#[derive(Debug)]
pub struct Chip {
    pub register: Register,
}

impl Chip {
    pub fn parse_op(op: u16) -> Instruction {
        let instruction = (op >> 12 & 15, (op >> 8) & 15, (op >> 4) & 15, op & 15);
        match instruction {
            (8, x, y, 0) => Instruction::Ld(x as usize, y as usize),
            (8, x, y, 1) => Instruction::Or(x as usize, y as usize),
            (8, x, y, 2) => Instruction::And(x as usize, y as usize),
            (8, x, y, 3) => Instruction::Xor(x as usize, y as usize),
            (8, x, y, 4) => Instruction::Add(x as usize, y as usize),
            (8, x, y, 5) => Instruction::Sub(x as usize, y as usize),
            (8, x, _, 6) => Instruction::Shr(x as usize),
            (8, x, y, 7) => Instruction::Subn(x as usize, y as usize),
            (8, x, _, 0xE) => Instruction::Shl(x as usize),
            _ => Instruction::Unknown,
        }
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Ld(vx, vy) => {
                let value = self.register.get(vy);
                self.register.set(vx, value);
            }
            Instruction::Add(vx, vy) => {
                let value_x = self.register.get(vx) as u16;
                let value_y = self.register.get(vy) as u16;
                let sum: u16 = value_x + value_y;
                let carry = if sum > 255 { 1 } else { 0 };
                self.register.set(0xF as usize, carry);
                self.register.set(vx, (sum & 255) as u8);
            }
            _ => (),
        }
    }

    #[allow(unused_variables)]
    pub fn execute(&mut self, instruction: u16) {
        let opcode = instruction >> 12;
        match opcode {
            6 => {
                // 6xkk - LD Vx, byte
                // Set Vx = kk
                // The interpreter puts the value kk into register Vx.
                let key = (instruction & 0xF00) >> 8;
                let value = instruction & 0xFF;
                self.register.set(key as usize, value as u8);
            }
            8 => {
                match instruction & 0xF {
                    0 => {
                        // 8xy0 - LD Vx, Vy
                        // Set Vx = Vy
                        // Stores the value of register Vy in register Vx.
                        let key_x = ((instruction & 0xF00) >> 8) as usize;
                        let key_y = ((instruction & 0xF0) >> 4) as usize;
                        let value = self.register.get(key_y);
                        self.register.set(key_x, value);
                    }
                    _ => (),
                }

            }
            _ => println!("{:?}", opcode),
        }
    }

    pub fn new() -> Chip {
        Chip { register: Register { values: [0u8; 16] } }
    }
}

fn main() {
    let mut chip = Chip::new();

    // LD 0, 57
    let opcode = (6 << 12) + (0 << 8) + 150;
    chip.execute(opcode);
    println!("{:?}", chip.register.values);

    // LD V1, V0
    let opcode = (8 << 12) + (1 << 8) + (0 << 4);
    chip.execute(opcode);
    println!("{:?}", chip.register.values);

    // LD V1, V3
    let opcode = (8 << 12) + (1 << 8) + (3 << 4);
    chip.execute(opcode);
    println!("{:?}", chip.register.values);

    println!("{:?}", Chip::parse_op(opcode));

    // LD V3, V0
    chip.execute_instruction(Instruction::Ld(3, 0));
    println!("{:?}", chip.register.values);

    // ADD V3, V0
    chip.execute_instruction(Instruction::Add(3, 0));
    println!("{:?}", chip.register.values);

    // ADD V3, V0
    chip.execute_instruction(Instruction::Add(3, 2));
    println!("{:?}", chip.register.values);
}
