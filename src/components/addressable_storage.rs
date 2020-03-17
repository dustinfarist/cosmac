pub trait AddressableStorage {
    fn set(&mut self, _: usize, _: u8);
    fn get(&self, _: usize) -> u8;
}
