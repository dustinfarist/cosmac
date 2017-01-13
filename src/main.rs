extern crate rand;

use rand::Rng;

#[derive(Debug)]
pub enum Instruction {
    /// 6xkk - LD Vx, byte
    /// Set Vx = kk.
    /// The interpreter puts the value kk into register Vx.
    LdByte(usize, u8),

    /// 7xkk - ADD Vx, byte
    /// Set Vx = Vx + kk.
    /// Adds the value kk to the value of register Vx, then stores the result in Vx.
    AddByte(usize, u8),
    
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

    /// Cxkk - RND Vx, byte
    /// Set Vx = random byte AND kk.
    /// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk.
    /// The results are stored in Vx. See instruction 8xy2 for more information on AND.
    Rnd(usize, u8),

    /// Unknown Instruction
    Unknown,
}

impl Instruction {
    pub fn parse(op: u16) -> Instruction {
        let instruction = (op >> 12 & 15, (op >> 8) & 15, (op >> 4) & 15, op & 15);
        match instruction {
            (6, x, a, b) => Instruction::LdByte(x as usize, (((a << 4) + b) & 255) as u8),
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
}

macro_rules! bitwise {
    ($a:expr, $op:tt, $b:expr) => (
        {
            let result = $a $op $b;
            println!("{:>8b} = {:?}", $a, $a);
            println!("{:>8b} = {:?}", $b, $b);
            println!("-------- = {:?}", stringify!($op));
            println!("{:>8b} = {:?}", result, result);
        }
    );
}

#[derive(Debug)]
pub struct Register {
    values: [u8; 16],
}

impl Register {
    pub fn new() -> Register {
        Register { values: [0u8; 16] }
    }

    pub fn with_values(values: &[u8]) -> Register {
        let mut vals = [0u8; 16];
        for i in 0..std::cmp::min(values.len(), 16) {
            vals[i] = values[i];
        }
        Register { values: vals }
    }

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
    pub fn execute(&mut self, instruction: &Instruction) {
        println!("{0:<15?} ", instruction);
        match *instruction {
            Instruction::LdByte(vx, value) => {
                self.register.set(vx, value)
            }
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
            Instruction::Rnd(vx, mask) => {
                let random: u8 = rand::thread_rng().gen::<u8>();
                self.register.set(vx, random & mask);
            }
            _ => (),
        }
        println!("{:?}\n", self.register.values);
    }

    pub fn new() -> Chip {
        Chip { register: Register::new() }
    }

    pub fn with_register_values(values: &[u8]) -> Chip {
        Chip { register: Register::with_values(values) }
    }
}

fn main() {
    let mut chip = Chip::new();
    let instructions = [
        Instruction::LdByte(0, 100),
        Instruction::Ld(1, 0),
        Instruction::Shl(0),
        Instruction::Shr(1),
        Instruction::Sub(0, 1),
        Instruction::Add(1, 0),
        Instruction::LdByte(2, 57),
        Instruction::Xor(1, 2),
        Instruction::Ld(3, 1),
        Instruction::And(3, 2),
        Instruction::Rnd(5, 255),
        Instruction::Rnd(5, 10), // Random number will always be less than 10.
    ];

    for ins in &instructions {
        chip.execute(ins);
    }
}

#[cfg(test)]
mod register_instructions {
    use {Chip, Instruction};

    macro_rules! register_eq {
        ($chip:tt, $vx:expr, $value:expr) => (assert_eq!($chip.register.get($vx), $value);)
    }

    #[test]
    fn load_byte() {
        let mut chip = Chip::new();
        chip.execute(&Instruction::LdByte(0, 10));
        chip.execute(&Instruction::LdByte(1, 15));
        register_eq!(chip, 0, 10);
        register_eq!(chip, 1, 15);
    }

    #[test]
    fn load_byte_overwrites_existing_value() {
        let mut chip = Chip::new();
        chip.execute(&Instruction::LdByte(0, 10));
        chip.execute(&Instruction::LdByte(0, 15));
        register_eq!(chip, 0, 15);
    }

    #[test]
    fn bitwise_and() {
        let mut chip = Chip::with_register_values(&[10, 15]);
        chip.execute(&Instruction::And(1, 0));
        register_eq!(chip, 1, 10 & 15);
    }

    #[test]
    fn bitwise_or() {
        let mut chip = Chip::with_register_values(&[10, 15]);
        chip.execute(&Instruction::Or(1, 0));
        register_eq!(chip, 1, 10 | 15);
    }

    #[test]
    fn bitwise_xor() {
        let mut chip = Chip::with_register_values(&[10, 15]);
        chip.execute(&Instruction::Xor(1, 0));
        register_eq!(chip, 1, 10 ^ 15);
    }

    #[test]
    fn add_works() {
        let mut chip = Chip::with_register_values(&[10, 15]);
        chip.execute(&Instruction::Add(1, 0));
        register_eq!(chip, 1, 25);
    }

    #[test]
    fn add_sets_carry_flag_on_overflow() {
        let mut chip = Chip::with_register_values(&[255, 1]);
        chip.execute(&Instruction::Add(0, 1));
        register_eq!(chip, 0, 0);
        register_eq!(chip, 1, 1);
        register_eq!(chip, 0xF, 1);
    }

    #[test]
    fn subtract_when_vx_greater_than_vy() {
        let mut chip = Chip::with_register_values(&[100, 25]);
        chip.execute(&Instruction::Sub(0, 1));
        register_eq!(chip, 0, 75);
        register_eq!(chip, 0xF, 1);
    }

    #[test]
    fn subtract_when_vx_less_than_vy() {
        let mut chip = Chip::with_register_values(&[25, 100]);
        chip.execute(&Instruction::Sub(0, 1));
        register_eq!(chip, 0, 181); // 256 + (-75)
        register_eq!(chip, 0xF, 0);
    }

    #[test]
    fn add_then_subtract_restores_state() {
        let mut chip = Chip::with_register_values(&[100, 25, 100]);
        chip.execute(&Instruction::Add(0, 1));
        register_eq!(chip, 0, 125);

        chip.execute(&Instruction::Sub(0, 1));
        register_eq!(chip, 0, 100);
    }

    #[test]
    fn add_then_subtract_restores_state_with_overflows() {
        let mut chip = Chip::with_register_values(&[255, 100, 100]);
        chip.execute(&Instruction::Add(0, 1));
        register_eq!(chip, 0, 99);
        register_eq!(chip, 0xF, 1);

        chip.execute(&Instruction::Sub(0, 1));
        register_eq!(chip, 0, 255);
        register_eq!(chip, 0xF, 0);
    }

    #[test]
    fn shift_right_with_odd_number_sets_vf_flag() {
        let mut chip = Chip::with_register_values(&[5]);
        chip.execute(&Instruction::Shr(0));
        register_eq!(chip, 0, 2);
        register_eq!(chip, 0xF, 1);
    }

    #[test]
    fn shift_right_with_even_number_does_not_set_vf_flag() {
        let mut chip = Chip::with_register_values(&[6]);
        chip.execute(&Instruction::Shr(0));
        register_eq!(chip, 0, 3);
        register_eq!(chip, 0xF, 0);
    }

    #[test]
    fn shift_left_then_shift_right_restores_state() {
        let mut chip = Chip::with_register_values(&[100]);
        chip.execute(&Instruction::Shl(0));
        register_eq!(chip, 0, 200);

        chip.execute(&Instruction::Shr(0));
        register_eq!(chip, 0, 100);

        chip.execute(&Instruction::Shr(0));
        register_eq!(chip, 0, 50);

        chip.execute(&Instruction::Shl(0));
        register_eq!(chip, 0, 100);
    }

    #[test]
    fn shift_right_then_shift_left_loses_info_with_odd_number() {
        let mut chip = Chip::with_register_values(&[5]);
        chip.execute(&Instruction::Shr(0));
        register_eq!(chip, 0, 2);
        register_eq!(chip, 0xF, 1);

        chip.execute(&Instruction::Shl(0));
        register_eq!(chip, 0, 4);
        register_eq!(chip, 0xF, 0);
    }

    #[test]
    fn shift_left_with_overflow_sets_vf_flag() {
        let mut chip = Chip::with_register_values(&[150]);
        chip.execute(&Instruction::Shl(0));
        register_eq!(chip, 0, 44);
        register_eq!(chip, 0xF, 1);
    }

    #[test]
    fn shift_left_then_shift_right_loses_info_with_overflow() {
        let mut chip = Chip::with_register_values(&[150]);
        chip.execute(&Instruction::Shl(0));
        register_eq!(chip, 0, 44);
        register_eq!(chip, 0xF, 1);

        chip.execute(&Instruction::Shr(0));
        register_eq!(chip, 0, 22);
        register_eq!(chip, 0xF, 0);
    }

    #[test]
    fn random_number_will_always_be_less_than_mask() {
        let mut chip = Chip::new();
        for _ in 0..100 {
            chip.execute(&Instruction::Rnd(0, 1));
            assert!(chip.register.get(0) <= 1);
        }
    }

    #[test]
    fn add_byte_works() {
        let mut chip = Chip::with_register_values(&[100]);
        chip.execute(&Instruction::AddByte(0, 100));
        register_eq!(chip, 0, 200);
    }

    #[test]
    fn add_byte_truncates_overflows() {
        let mut chip = Chip::with_register_values(&[255]);
        chip.execute(&Instruction::AddByte(0, 100));
        register_eq!(chip, 0, 99);
    }
}
