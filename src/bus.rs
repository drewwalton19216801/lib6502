/// The `Bus` trait defines the interface for memory operations.
/// It can be implemented by any type that wants to handle memory reads and writes.
pub trait Bus {
    /// Read a byte from the specified address.
    fn read(&mut self, addr: u16) -> u8;

    /// Write a byte to the specified address.
    fn write(&mut self, addr: u16, data: u8);
}
