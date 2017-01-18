use components::AddressableStorage;

pub struct Memory {
    pub values: [u8; 4096],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { values: [0u8; 4096] }
    }

    pub fn with_values(values: &[u8]) -> Memory {
        let mut vals = [0u8; 4096];
        for i in 0..::std::cmp::min(values.len(), 4096) {
            vals[i] = values[i];
        }
        Memory { values: vals }
    }
}

impl AddressableStorage for Memory {
    fn set(&mut self, key: usize, value: u8) {
        self.values[key] = value;
    }

    fn get(&self, key: usize) -> u8 {
        self.values[key]
    }
}
