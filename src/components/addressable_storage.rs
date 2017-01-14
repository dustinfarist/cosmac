pub trait AddressableStorage {
    fn set(&mut self, usize, u8);
    fn get(&self, usize) -> u8;
}
