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

struct NullBus;

impl Bus for NullBus {
    fn read(&self, _address: u16) -> u8 {
        0
    }
    fn write(&mut self, _address: u16, _value: u8) {}
    fn load(&mut self, _path: &str, _address: u16) {}
}

/// Create a null bus
pub fn null() -> Box<dyn Bus> {
    Box::new(NullBus)
}
