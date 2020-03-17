use crate::components::AddressableStorage;

#[derive(Debug)]
pub struct Register {
    pub delay: u8,
    pub i: u16,
    pub sound: u8,
    pub values: [u8; 16],
}

impl Register {
    pub fn new() -> Register {
        Register {
            delay: 0,
            i: 0,
            sound: 0,
            values: [0u8; 16],
        }
    }

    pub fn with_values(values: &[u8]) -> Register {
        let mut vals = [0u8; 16];
        for i in 0..::std::cmp::min(values.len(), 16) {
            vals[i] = values[i];
        }
        Register {
            delay: 0,
            i: 0,
            sound: 0,
            values: vals,
        }
    }
}

impl AddressableStorage for Register {
    fn set(&mut self, key: usize, value: u8) {
        self.values[key] = value;
    }

    fn get(&self, key: usize) -> u8 {
        self.values[key]
    }
}
