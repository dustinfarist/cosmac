#[derive(Debug)]
pub struct Register {
    values: [u8; 16]
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
    pub register: Register
}

impl Chip {
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
            },
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
        Chip {
            register: Register { values: [0u8; 16] }
        }
    }
}

fn main() {
    let mut chip = Chip::new();

    // LD 0, 57
    let opcode = (6 << 12) + (0 << 8) + 57;
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
}
