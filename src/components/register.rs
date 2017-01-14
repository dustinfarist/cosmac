use components::AddressableStorage;

#[derive(Debug)]
pub struct Register {
    pub values: [u8; 16],
}

impl Register {
    pub fn new() -> Register {
        Register { values: [0u8; 16] }
    }

    pub fn with_values(values: &[u8]) -> Register {
        let mut vals = [0u8; 16];
        for i in 0..::std::cmp::min(values.len(), 16) {
            vals[i] = values[i];
        }
        Register { values: vals }
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
