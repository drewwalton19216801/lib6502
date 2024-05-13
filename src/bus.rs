/// The bus module
///
/// A bus is a device that can read and write to memory. It is connected to the CPU
/// and other devices.
pub trait Bus {
    /// Read a byte from memory
    fn read(&self, address: u16) -> u8;
    /// Write a byte to memory
    fn write(&mut self, address: u16, value: u8);
    /// Load a binary file into memory
    fn load(&mut self, path: &str, address: u16);
}