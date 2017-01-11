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
    pub fn execute(&self, instruction: u16) {

    }

    pub fn new() -> Chip {
        Chip {
            register: Register { values: [0u8; 16] }
        }
    }
}

fn main() {
    let mut register = Register { values: [0; 16] };
    register.set(3, 5);
    println!("{:?}", register.get(3));

    let mut chip = Chip::new();
    chip.register.set(3, 5);
    println!("{:?}", chip);
}
